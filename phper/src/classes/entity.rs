use std::ffi::c_char;
use std::{any::Any, marker::PhantomData, mem::zeroed, rc::Rc};

use smallvec::SmallVec;

use phper_sys::{phper_register_class_entry, zend_class_entry, zend_function_entry};

use crate::classes::methods::MethodEntityBuilder;
use crate::errors::Throwable;
use crate::functions::{Function, Method};
use crate::{functions::FunctionEntry, objects::StateObj, values::ZVal};

use super::{create_object, StateConstructor, StaticStateClass};

/// Builder for registering class.
///
/// *It is a common practice for PHP extensions to use PHP objects to package
/// third-party resources.*
pub struct ClassEntity<T> {
    state_constructor: Rc<StateConstructor>,
    method_entities: SmallVec<[FunctionEntry; 16]>,
    bind_class: Option<&'static StaticStateClass<()>>,
    class_create: unsafe extern "C" fn() -> *mut zend_class_entry,
    // state_cloner: Option<Rc<StateCloner>>,
    _p: PhantomData<*mut T>,
}

impl ClassEntity<()> {
    /// Construct a new `ClassEntity` with class name, do not own state.
    pub fn new(class_create: unsafe extern "C" fn() -> *mut zend_class_entry) -> Self {
        Self::new_with_state_constructor(|| (), class_create)
    }
}

impl<T> ClassEntity<T> {
    /// Construct a new `ClassEntity` with class name and default state
    /// constructor.
    pub fn new_with_default_state_constructor(
        class_create: unsafe extern "C" fn() -> *mut zend_class_entry,
    ) -> Self
    where
        T: Default + 'static,
    {
        Self::new_with_state_constructor(T::default, class_create)
    }
}

pub trait Handler<Z, E> {
    fn execute(&self, state: &mut StateObj, args: &mut [ZVal]) -> Result<Z, E>;
}

impl<Z, E> Handler<Z, E> for dyn Fn(&mut StateObj, &mut [ZVal]) -> Result<Z, E> + 'static {
    fn execute(&self, state: &mut StateObj, args: &mut [ZVal]) -> Result<Z, E> {
        self(state, args)
    }
}

impl<T> ClassEntity<T> {
    /// Construct a new `ClassEntity` with class name and the constructor to
    /// build state.
    pub fn new_with_state_constructor(
        state_constructor: impl Fn() -> T + 'static,
        class_create: unsafe extern "C" fn() -> *mut zend_class_entry,
    ) -> Self
    where
        T: 'static,
    {
        Self {
            state_constructor: Rc::new(move || Box::new(state_constructor()) as Box<dyn Any>),
            method_entities: SmallVec::default(),
            class_create,
            // state_cloner: None,
            bind_class: None,
            _p: Default::default(),
        }
    }

    /// Add member method to class, with visibility and method handler.
    pub fn add_method<F, Z, E>(&mut self, handler: F, method_builder: MethodEntityBuilder)
    where
        F: Fn(&mut StateObj, &mut [ZVal]) -> Result<Z, E> + 'static,
        Z: Into<ZVal> + 'static,
        E: Throwable + 'static,
    {
        let entity = method_builder
            .set_handler(Method::<F, Z, E>::new(handler))
            .build();
        self.method_entities.push(entity.into());
    }

    /// Add static method to class, with visibility and method handler.
    pub fn add_static_method<F, Z, E>(&mut self, handler: F, method_builder: MethodEntityBuilder)
    where
        F: Fn(&mut [ZVal]) -> Result<Z, E> + 'static,
        Z: Into<ZVal> + 'static,
        E: Throwable + 'static,
    {
        let entity = method_builder
            .set_abstract()
            .set_handler(Function::new(handler))
            .build();
        self.method_entities.push(entity.into());
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
        // self.state_cloner = Some(Rc::new(move |src| {
        //     let src = unsafe {
        //         src.as_ref()
        //             .unwrap()
        //             .downcast_ref::<T>()
        //             .expect("cast Any to T failed")
        //     };
        //     let dest = clone_fn(src);
        //     let boxed = Box::new(dest) as Box<dyn Any>;
        //     Box::into_raw(boxed)
        // }));
    }
}

impl<T> crate::modules::Registerer for ClassEntity<T> {
    fn register(mut self, _: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let mut methods = std::mem::take(&mut self.method_entities);
            methods.push(FunctionEntry::empty());

            {
                let mut entry = zeroed::<zend_function_entry>();

                let ptr = std::ptr::from_mut(&mut entry.fname);
                let state_constructor = Rc::into_raw(self.state_constructor) as *const c_char;
                ptr.write(state_constructor);
                methods.push(FunctionEntry(entry));
            }

            // Store the state constructor pointer to zend_class_entry.

            // if let Some(state_cloner) = self.state_cloner {
            //     let mut entry = zeroed::<zend_function_entry>();
            //     let ptr = &mut entry as *mut _ as *mut *const StateCloner;
            //     let state_constructor = Rc::into_raw(state_cloner.clone());
            //     ptr.write(state_constructor);
            //     methods.push(FunctionEntry(entry));
            // }

            let class_ce = phper_register_class_entry(
                Some(self.class_create),
                Box::into_raw(methods.into_boxed_slice()).cast(),
                Some(create_object),
            );

            if let Some(bind_class) = self.bind_class {
                bind_class.bind(class_ce);
            }
        }

        Ok(())
    }
}
