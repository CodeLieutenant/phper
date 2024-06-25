use crate::classes::entry::ClassEntry;
use crate::classes::methods::MethodEntityBuilder;
use crate::functions::FunctionEntry;
use phper_sys::{
    phper_init_interface_entry, phper_register_interface_entry, zend_class_entry,
    zend_class_implements,
};
use smallvec::SmallVec;
use std::mem::zeroed;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

#[repr(transparent)]
pub struct StaticInterface(AtomicPtr<zend_class_entry>);

impl StaticInterface {
    /// Create empty [StaticInterface], with null
    /// [zend_class_entry].
    pub const fn null() -> Self {
        Self(AtomicPtr::new(null_mut()))
    }

    fn bind(&'static self, ptr: *mut zend_class_entry) {
        self.0.store(ptr, Ordering::Relaxed);
    }

    /// Converts to class entry.
    pub fn as_class_entry(&'static self) -> &'static ClassEntry {
        unsafe { ClassEntry::from_mut_ptr(self.0.load(Ordering::Relaxed)) }
    }
}

/// Builder for registering interface.
pub struct InterfaceEntity {
    interface: zend_class_entry,
    methods: SmallVec<[FunctionEntry; 16]>,
    extends: SmallVec<[Box<dyn Fn() -> &'static ClassEntry>; 1]>,
    bind_interface: Option<&'static StaticInterface>,
}

impl InterfaceEntity {
    /// Construct a new `InterfaceEntity` with interface name.
    pub fn new(interface_name: impl AsRef<str>) -> Self {
        let interface_name = interface_name.as_ref();
        let interface_name_len = interface_name.len();

        Self {
            interface: unsafe {
                phper_init_interface_entry(interface_name.as_ptr().cast(), interface_name_len)
            },
            methods: SmallVec::default(),
            extends: SmallVec::default(),
            bind_interface: None,
        }
    }

    /// Add member method to interface, with mandatory visibility public
    /// abstract.
    pub fn add_method(&mut self, builder: MethodEntityBuilder) {
        self.methods.push(builder.set_abstract().build().into());
    }

    /// Register interface to `extends` the interfaces, due to the interface can
    /// extend multi interface, so this method can be called multi time.
    ///
    /// *Because in the `MINIT` phase, the class starts to register, so the*
    /// *closure is used to return the `ClassEntry` to delay the acquisition of*
    /// *the class.*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::{ClassEntry, InterfaceEntity};
    ///
    /// let mut interface = InterfaceEntity::new("MyInterface");
    /// interface.extends(|| ClassEntry::from_globals("Stringable").unwrap());
    /// ```
    pub fn extends(&mut self, interface: impl Fn() -> &'static ClassEntry + 'static) {
        self.extends.push(Box::new(interface));
    }

    /// Bind to static [StaticInterface].
    ///
    /// When the interface registered, the [StaticInterface] will be
    /// initialized, so you can use the [StaticInterface] to be implemented
    /// by other class.
    pub fn bind(&mut self, i: &'static StaticInterface) {
        self.bind_interface = Some(i);
    }
}

impl crate::modules::Registerer for InterfaceEntity {
    fn register(mut self, _: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            self.methods.push(zeroed::<FunctionEntry>());

            let class_ce =
                phper_register_interface_entry(&mut self.interface, self.methods.as_ptr().cast());

            if let Some(bind_interface) = self.bind_interface {
                bind_interface.bind(class_ce);
            }

            for interface in &self.extends {
                let interface_ce = interface().as_ptr();
                zend_class_implements(class_ce, 1, interface_ce);
            }
        };

        Ok(())
    }
}
