use smallvec::SmallVec;
use std::{any::Any, marker::PhantomData, mem::zeroed, ptr::null_mut, rc::Rc};

use phper_sys::{
    phper_init_class_entry, phper_register_class_entry, zend_class_entry, zend_class_implements,
    zend_function_entry,
};

use crate::{functions::FunctionEntry, objects::StateObj, types::Scalar, values::ZVal};

use super::{
    create_object, entry::ClassEntry, PropertyEntity, StateCloner, StateConstructor,
    StaticStateClass, Visibility,
};

/// Builder for registering class.
///
/// *It is a common practice for PHP extensions to use PHP objects to package
/// third-party resources.*
pub struct ClassEntity<T> {
    class: zend_class_entry,
    state_constructor: Rc<StateConstructor<T>>,
    method_entities: SmallVec<[FunctionEntry; 16]>,
    property_entities: Vec<PropertyEntity>,
    parent: Option<Box<dyn Fn() -> &'static ClassEntry>>,
    interfaces: Vec<Box<dyn Fn() -> &'static ClassEntry>>,
    bind_class: Option<&'static StaticStateClass<()>>,
    state_cloner: Option<Rc<StateCloner>>,
    _p: PhantomData<*mut T>,
}

impl ClassEntity<()> {
    /// Construct a new `ClassEntity` with class name, do not own state.
    pub fn new(class_name: impl AsRef<str>) -> Self {
        Self::new_with_state_constructor(class_name, || ())
    }
}

impl<T> ClassEntity<T> {
    /// Construct a new `ClassEntity` with class name and default state
    /// constructor.
    pub fn new_with_default_state_constructor(class_name: impl AsRef<str>) -> Self
    where
        T: Default + 'static,
    {
        Self::new_with_state_constructor(class_name, T::default)
    }
}

pub trait Handler<T, Z, E> {
    fn execute(&self, state: &mut StateObj<T>, args: &mut [ZVal]) -> Result<Z, E>;
}

impl<T, Z, E> Handler<T, Z, E> for dyn Fn(&mut StateObj<T>, &mut [ZVal]) -> Result<Z, E> + 'static {
    fn execute(&self, state: &mut StateObj<T>, args: &mut [ZVal]) -> Result<Z, E> {
        self(state, args)
    }
}

impl<T> ClassEntity<T> {
    /// Construct a new `ClassEntity` with class name and the constructor to
    /// build state.
    pub fn new_with_state_constructor(
        class_name: impl AsRef<str>,
        state_constructor: impl Fn() -> T + 'static,
    ) -> Self
    where
        T: 'static,
    {
        let class_name = class_name.as_ref();
        let class_name_len = class_name.len();

        Self {
            class: unsafe { phper_init_class_entry(class_name.as_ptr().cast(), class_name_len) },
            state_constructor: Rc::new(state_constructor),
            method_entities: SmallVec::default(),
            property_entities: Vec::new(),
            parent: None,
            interfaces: Vec::new(),
            state_cloner: None,
            bind_class: None,
            _p: Default::default(),
        }
    }

    /// Add member method to class, with visibility and method handler.
    // pub fn add_method<F, Z, E>(
    //     &mut self,
    //     name: impl AsRef<str>,
    //     vis: Visibility,
    //     handler: F,
    // ) -> &mut MethodEntity
    // where
    //     F: Fn(&mut StateObj, &mut [ZVal]) -> Result<Z, E> + 'static,
    //     Z: Into<ZVal> + 'static,
    //     E: Throwable + 'static,
    // {
    //     self.method_entities.push(MethodEntity::new(
    //         name,
    //         Some(Rc::new(Method::<F, Z, E>::new(handler))),
    //         vis,
    //         null(),
    //     ));
    //     self.method_entities.last_mut().unwrap()
    // }

    /// Add static method to class, with visibility and method handler.
    // pub fn add_static_method<F, Z, E>(
    //     &mut self,
    //     name: impl AsRef<str>,
    //     vis: Visibility,
    //     handler: F,
    // ) -> &mut MethodEntity
    // where
    //     F: Fn(&mut [ZVal]) -> Result<Z, E> + 'static,
    //     Z: Into<ZVal> + 'static,
    //     E: Throwable + 'static,
    // {
    //     let mut entity = MethodEntity::new(name, Some(Rc::new(Function::new(handler))), vis, null());
    //     entity.set_vis_static();
    //     self.method_entities.push(entity);
    //     self.method_entities.last_mut().unwrap()
    // }

    /// Add abstract method to class, with visibility (shouldn't be private).
    // pub fn add_abstract_method(
    //     &mut self,
    //     name: impl AsRef<str>,
    //     vis: Visibility,
    // ) -> &mut MethodEntity {
    //     let mut entity = MethodEntity::new(name, None, vis, null());
    //     entity.set_vis_abstract();
    //     self.method_entities.push(entity);
    //     self.method_entities.last_mut().unwrap()
    // }

    /// Declare property.
    ///
    /// The argument `value` should be `Copy` because 'zend_declare_property'
    /// receive only scalar zval , otherwise will report fatal error:
    /// "Internal zvals cannot be refcounted".
    pub fn add_property(
        &mut self,
        name: impl Into<String>,
        visibility: Visibility,
        value: impl Into<Scalar>,
    ) {
        self.property_entities
            .push(PropertyEntity::new(name, visibility, value));
    }

    /// Declare static property.
    ///
    /// The argument `value` should be `Copy` because 'zend_declare_property'
    /// receive only scalar zval , otherwise will report fatal error:
    /// "Internal zvals cannot be refcounted".
    pub fn add_static_property(
        &mut self,
        name: impl Into<String>,
        visibility: Visibility,
        value: impl Into<Scalar>,
    ) {
        let mut entity = PropertyEntity::new(name, visibility, value);
        entity.set_vis_static();
        self.property_entities.push(entity);
    }

    /// Register class to `extends` the parent class.
    ///
    /// *Because in the `MINIT` phase, the class starts to register, so the*
    /// *closure is used to return the `ClassEntry` to delay the acquisition of*
    /// *the class.*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::{ClassEntity, ClassEntry};
    ///
    /// let mut class = ClassEntity::new("MyException");
    /// class.extends(|| ClassEntry::from_globals("Exception").unwrap());
    /// ```
    pub fn extends(&mut self, parent: impl Fn() -> &'static ClassEntry + 'static) {
        self.parent = Some(Box::new(parent));
    }

    /// Register class to `implements` the interface, due to the class can
    /// implement multi interface, so this method can be called multi time.
    ///
    /// *Because in the `MINIT` phase, the class starts to register, so the*
    /// *closure is used to return the `ClassEntry` to delay the acquisition of*
    /// *the class.*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::{ClassEntity, ClassEntry};
    ///
    /// let mut class = ClassEntity::new("MyClass");
    /// class.implements(|| ClassEntry::from_globals("Stringable").unwrap());
    ///
    /// // Here you have to the implement the method `__toString()` in `Stringable`
    /// // for `MyClass`, otherwise `MyClass` will become abstract class.
    /// // ...
    /// ```
    pub fn implements(&mut self, interface: impl Fn() -> &'static ClassEntry + 'static) {
        self.interfaces.push(Box::new(interface));
    }

    /// Bind to static [StaticStateClass].
    ///
    /// When the class registered, the [StaticStateClass] will be initialized,
    /// so you can use the [StaticStateClass] to new stateful object, etc.
    pub fn bind(&mut self, cls: &'static StaticStateClass<T>) {
        self.bind_class = Some(unsafe {
            std::mem::transmute::<&'static StaticStateClass<T>, &'static StaticStateClass<()>>(cls)
        });
    }

    /// Add the state clone function, called when cloning PHP object.
    ///
    /// By default, the object registered by `phper` is uncloneable, if you
    /// clone the object in PHP like this:
    ///
    /// ```php
    /// $foo = new Foo();
    /// $foo2 = clone $foo;
    /// ```
    ///
    /// Will throw the Error: `Uncaught Error: Trying to clone an uncloneable
    /// object of class Foo`.
    ///
    /// And then, if you want the object to be cloneable, you should register
    /// the state clone method for the class.
    ///
    /// # Examples
    ///
    /// ```
    /// use phper::classes::ClassEntity;
    ///
    /// fn make_foo_class<T>() -> ClassEntity<T> {
    ///     let mut class = ClassEntity::new_with_state_constructor("Foo", || 123456);
    ///     class.state_cloner(Clone::clone);
    ///     class
    /// }
    /// ```
    pub fn state_cloner(&mut self, clone_fn: impl Fn(&T) -> T + 'static)
    where
        T: 'static,
    {
        self.state_cloner = Some(Rc::new(move |src| {
            let src = unsafe {
                src.as_ref()
                    .unwrap()
                    .downcast_ref::<T>()
                    .expect("cast Any to T failed")
            };
            let dest = clone_fn(src);
            let boxed = Box::new(dest) as Box<dyn Any>;
            Box::into_raw(boxed)
        }));
    }
}

impl<T> crate::modules::Registerer for ClassEntity<T> {
    fn register(mut self, _: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let parent: *mut zend_class_entry = self
                .parent
                .as_ref()
                .map(|parent| parent().as_ptr() as *mut _)
                .unwrap_or(null_mut());

            let mut methods = std::mem::take(&mut self.method_entities);
            methods.push(FunctionEntry::empty());

            {
                // let mut entry = zeroed::<zend_function_entry>();
                // let ptr = &mut entry as *mut _ as *mut *const StateConstructor;
                // let state_constructor = Rc::into_raw(self.state_constructor);
                // ptr.write(state_constructor);
                // methods.push(FunctionEntry(entry));
            }

            // Store the state constructor pointer to zend_class_entry.

            if let Some(state_cloner) = self.state_cloner {
                let mut entry = zeroed::<zend_function_entry>();
                let ptr = &mut entry as *mut _ as *mut *const StateCloner;
                let state_constructor = Rc::into_raw(state_cloner.clone());
                ptr.write(state_constructor);
                methods.push(FunctionEntry(entry));
            }

            // Store the state cloner pointer to zend_class_entry.

            let class_ce =
                phper_register_class_entry(&mut self.class, parent, methods.as_ptr().cast());

            if let Some(bind_class) = self.bind_class {
                bind_class.bind(class_ce);
            }

            for interface in self.interfaces {
                let interface_ce = interface().as_ptr();
                zend_class_implements(class_ce, 1, interface_ce);
            }

            (*class_ce).__bindgen_anon_2.create_object = Some(create_object);

            for property in &self.property_entities {
                property.declare(class_ce);
            }
        }

        Ok(())
    }
}
