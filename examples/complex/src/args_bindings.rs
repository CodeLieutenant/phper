
use phper::zend_create_fn;


#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(deref_nullptr)]
#[allow(clippy::all)]
mod bindings {

    include!(concat!(env!("OUT_DIR"), "/php_args_bindings.rs"));
}

use bindings::register_class_Complex_Foo;

pub use bindings::{
    arginfo_Complex_say_hello, arginfo_Complex_throw_exception,
    arginfo_class_Complex_Foo_getFoo, arginfo_class_Complex_Foo_setFoo,
};

zend_create_fn!(register_class_Complex_Foo, CLASS_COMPLEX_FOO);
