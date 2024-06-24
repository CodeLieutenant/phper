// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

mod args_bindings;

use std::convert::Infallible;
use std::ffi::CStr;

use args_bindings::{
    arginfo_Complex_get_all_ini, arginfo_Complex_say_hello, arginfo_Complex_throw_exception,
    arginfo_class_Complex_Foo_getFoo, arginfo_class_Complex_Foo_setFoo,
};

use crate::args_bindings::CLASS_COMPLEX_FOO;
use phper::arrays::ZArray;
use phper::classes::methods::MethodEntityBuilder;
use phper::classes::ClassEntity;
use phper::ini::{ini_get, Policy};
use phper::objects::StateObj;
use phper::{modules::Module, php_get_module, values::ZVal, zend_args};

fn say_hello(arguments: &mut [ZVal]) -> phper::Result<String> {
    let name = &mut arguments[0];
    name.convert_to_string();
    let name = name.as_z_str().unwrap().to_str()?;

    Ok(format!("Hello, {name}!\n"))
}

fn throw_exception(_: &mut [ZVal]) -> phper::Result<()> {
    Err(phper::Error::Boxed("I am sorry".into()))
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // register module ini
    module.add_ini("complex.enable", false, Policy::All);
    module.add_ini("complex.num", 100, Policy::All);
    module.add_ini("complex.ratio", 1.5, Policy::All);
    module.add_ini(
        "complex.description",
        "hello world.".to_owned(),
        Policy::All,
    );

    // register hook functions
    module.on_module_init(|_info| {});
    module.on_module_shutdown(|_info| {});
    module.on_request_init(|_info| {});
    module.on_request_shutdown(|_info| {});

    module.add_function(
        "Complex\\say_hello",
        zend_args!(arginfo_Complex_say_hello),
        say_hello,
    );
    // .add_function(
    //     "Complex\\throw_exception",
    //     zend_args!(arginfo_Complex_throw_exception),
    //     throw_exception,
    // )
    // .add_function(
    //     "Complex\\get_all_ini",
    //     zend_args!(arginfo_Complex_get_all_ini),
    //     |_: &mut [ZVal]| {
    //         let mut arr = ZArray::new();
    //
    //         let complex_enable = ZVal::from(ini_get::<bool>("complex.enable"));
    //         arr.insert("complex.enable", complex_enable);
    //
    //         let complex_description =
    //             ZVal::from(ini_get::<Option<&CStr>>("complex.description"));
    //         arr.insert("complex.description", complex_description);
    //         Ok::<_, Infallible>(arr)
    //     },
    // );
    //
    // let mut foo_class = ClassEntity::new(CLASS_COMPLEX_FOO);
    //
    // foo_class.add_method(
    //     |this: &mut StateObj, _: &mut [ZVal]| {
    //         Ok::<_, phper::Error>(this.get_property("foo").clone())
    //     },
    //     MethodEntityBuilder::new("getFoo", zend_args!(arginfo_class_Complex_Foo_getFoo))
    //         .set_public(),
    // );
    //
    // foo_class.add_method(
    //     |this: &mut StateObj, arguments: &mut [ZVal]| -> phper::Result<()> {
    //         this.set_property("foo", arguments[0].clone());
    //         Ok(())
    //     },
    //     MethodEntityBuilder::new("setFoo", zend_args!(arginfo_class_Complex_Foo_setFoo))
    //         .set_public(),
    // );
    // module.add_class(foo_class);

    module.add_info("extra info key", "extra info value");

    module
}
