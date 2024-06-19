// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_function_entry].
//!
//! TODO Add lambda.

use crate::{
    classes::{entry::ClassEntry, RawVisibility, Visibility},
    errors::{throw, ArgumentCountError, ExceptionGuard, ThrowObject, Throwable},
    objects::{StateObj, ZObj, ZObject},
    strings::{ZStr, ZString},
    sys::*,
    utils::ensure_end_with_zero,
    values::{ExecuteData, ZVal},
};
use phper_alloc::ToRefOwned;
use std::mem::zeroed;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::transmute,
    ptr::{self, null_mut},
    rc::Rc,
};
use crate::classes::MethodEntity;

pub(crate) trait Callable {
    fn call(&self, execute_data: &mut ExecuteData, arguments: &mut [ZVal], return_value: &mut ZVal);
}

pub(crate) struct Function<F, Z, E>(F, PhantomData<(Z, E)>);

impl<F, Z, E> Function<F, Z, E> {
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Z, E> Callable for Function<F, Z, E>
where
    F: Fn(&mut [ZVal]) -> Result<Z, E>,
    Z: Into<ZVal>,
    E: Throwable,
{
    fn call(&self, _: &mut ExecuteData, arguments: &mut [ZVal], return_value: &mut ZVal) {
        match (self.0)(arguments) {
            Ok(z) => {
                *return_value = z.into();
            }
            Err(e) => {
                unsafe {
                    throw(e);
                }
                *return_value = ().into();
            }
        }
    }
}

pub(crate) struct Method<F, Z, E>(F, PhantomData<(Z, E)>);

impl<F, Z, E> Method<F, Z, E> {
    pub(crate) fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Z, E> Callable for Method<F, Z, E>
where
    F: Fn(&mut StateObj, &mut [ZVal]) -> Result<Z, E>,
    Z: Into<ZVal>,
    E: Throwable,
{
    fn call(
        &self,
        execute_data: &mut ExecuteData,
        arguments: &mut [ZVal],
        return_value: &mut ZVal,
    ) {
        let this = unsafe { execute_data.get_this_mut().unwrap().as_mut_state_obj() };
        match (self.0)(this, arguments) {
            Ok(z) => {
                *return_value = z.into();
            }
            Err(e) => {
                unsafe {
                    throw(e);
                }
                *return_value = ().into();
            }
        }
    }
}

/// Wrapper of [`zend_function_entry`].
#[repr(C)]
pub struct FunctionEntry(pub(crate) zend_function_entry);

impl FunctionEntry {
    pub(crate) fn empty() -> Self {
        Self(unsafe { zeroed::<zend_function_entry>() })
    }

    pub(crate) unsafe fn from_function_entity(entity: FunctionEntity) -> FunctionEntry {
        Self::entry(
            &entity.name,
            entity.arguments,
            Some(entity.handler.clone()),
            None,
        )
    }

    pub(crate) unsafe fn from_method_entity(entity:  MethodEntity) -> FunctionEntry {
        Self::entry(
            &entity.name,
            entity.arguments,
            entity.handler.clone(),
            Some(entity.visibility),
        )
    }

    unsafe fn entry(
        name: &CStr,
        arguments: &'static [zend_internal_arg_info],
        handler: Option<Rc<dyn Callable>>,
        visibility: Option<RawVisibility>,
    ) -> FunctionEntry {
        let raw_handler = handler.as_ref().map(|_| invoke as _);

        if let Some(handler) = handler {
            let translator = CallableTranslator {
                callable: Rc::into_raw(handler),
            };
            let last_arg_info: zend_internal_arg_info = translator.internal_arg_info;
            // infos.push(last_arg_info);
        }

        let flags = visibility.unwrap_or(Visibility::default() as u32);

        FunctionEntry(zend_function_entry {
            fname: name.as_ptr().cast(),
            handler: raw_handler,
            arg_info: null_mut(),
            num_args: 0u32,
            flags,
        })
    }
}

/// Builder for registering php function.
pub struct FunctionEntity {
    name: CString,
    handler: Rc<dyn Callable>,
    arguments: &'static [zend_internal_arg_info],
}

impl FunctionEntity {
    #[inline]
    pub(crate) fn new(
        name: impl AsRef<str>,
        handler: Rc<dyn Callable>,
        arguments: &'static [zend_internal_arg_info],
    ) -> Self {
        FunctionEntity {
            name: ensure_end_with_zero(name),
            handler,
            arguments,
        }
    }
}


/// Wrapper of [`zend_function`].
#[repr(transparent)]
pub struct ZFunc {
    inner: zend_function,
}

impl ZFunc {
    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    pub(crate) unsafe fn from_mut_ptr<'a>(ptr: *mut zend_function) -> &'a mut ZFunc {
        let ptr = ptr as *mut Self;
        ptr.as_mut().expect("ptr shouldn't be null")
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_function {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_function {
        &mut self.inner
    }

    /// Get the function name if exists.
    pub fn get_function_name(&self) -> Option<&ZStr> {
        unsafe {
            let s = phper_get_function_name(self.as_ptr());
            ZStr::try_from_ptr(s)
        }
    }

    /// Get the function or method fully-qualified name.
    pub fn get_function_or_method_name(&self) -> ZString {
        unsafe {
            let s = phper_get_function_or_method_name(self.as_ptr());
            ZString::from_raw(s)
        }
    }

    /// Get the function related class if exists.
    pub fn get_class(&self) -> Option<&ClassEntry> {
        unsafe {
            let ptr = self.inner.common.scope;
            if ptr.is_null() {
                None
            } else {
                Some(ClassEntry::from_ptr(self.inner.common.scope))
            }
        }
    }

    #[allow(clippy::useless_conversion)]
    pub(crate) fn call(
        &mut self,
        mut object: Option<&mut ZObj>,
        mut arguments: impl AsMut<[ZVal]>,
    ) -> crate::Result<ZVal> {
        let arguments = arguments.as_mut();
        let function_handler = self.as_mut_ptr();

        let object_ptr = object
            .as_mut()
            .map(|o| o.as_mut_ptr())
            .unwrap_or(null_mut());

        call_raw_common(|ret| unsafe {
            let class_ptr = object
                .as_mut()
                .map(|o| o.get_mut_class().as_mut_ptr())
                .unwrap_or(null_mut());

            zend_call_known_function(
                function_handler,
                object_ptr,
                class_ptr,
                ret.as_mut_ptr(),
                arguments.len() as u32,
                arguments.as_mut_ptr().cast(),
                null_mut(),
            );
        })
    }
}

/// Just for type transmutation.
pub(crate) union CallableTranslator {
    pub(crate) callable: *const dyn Callable,
    pub(crate) internal_arg_info: zend_internal_arg_info,
    pub(crate) arg_info: zend_arg_info,
}

/// The entry for all registered PHP functions.
unsafe extern "C" fn invoke(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    let execute_data = ExecuteData::from_mut_ptr(execute_data);
    let return_value = ZVal::from_mut_ptr(return_value);

    let num_args = execute_data.common_num_args();
    let arg_info = execute_data.common_arg_info();

    let last_arg_info = arg_info.offset((num_args + 1) as isize);
    let translator = CallableTranslator {
        arg_info: *last_arg_info,
    };
    let handler = translator.callable;
    let handler = handler.as_ref().expect("handler is null");

    // Check arguments count.
    let num_args = execute_data.num_args();
    let required_num_args = execute_data.common_required_num_args();
    if num_args < required_num_args {
        let func_name = execute_data.func().get_function_or_method_name();
        let err: crate::Error = match func_name.to_str() {
            Ok(func_name) => {
                ArgumentCountError::new(func_name.to_owned(), required_num_args, num_args).into()
            }
            Err(e) => e.into(),
        };
        throw(err);
        *return_value = ().into();
        return;
    }

    let mut arguments = execute_data.get_parameters_array();
    let arguments = arguments.as_mut_slice();

    handler.call(execute_data, transmute(arguments), return_value);
}

/// Call user function by name.
///
/// # Examples
///
/// ```no_run
/// use phper::{arrays::ZArray, functions::call, values::ZVal};
///
/// fn json_encode() -> phper::Result<()> {
///     let mut arr = ZArray::new();
///     arr.insert("a", ZVal::from(1));
///     arr.insert("b", ZVal::from(2));
///     let ret = call("json_encode", &mut [ZVal::from(arr)])?;
///     assert_eq!(ret.expect_z_str()?.to_str(), Ok(r#"{"a":1,"b":2}"#));
///     Ok(())
/// }
/// ```
pub fn call(callable: impl Into<ZVal>, arguments: impl AsMut<[ZVal]>) -> crate::Result<ZVal> {
    let mut func = callable.into();
    call_internal(&mut func, None, arguments)
}

pub(crate) fn call_internal(
    func: &mut ZVal,
    mut object: Option<&mut ZObj>,
    mut arguments: impl AsMut<[ZVal]>,
) -> crate::Result<ZVal> {
    let func_ptr = func.as_mut_ptr();
    let arguments = arguments.as_mut();

    let mut object_val = object.as_mut().map(|obj| ZVal::from(obj.to_ref_owned()));

    call_raw_common(|ret| unsafe {
        phper_call_user_function(
            object_val
                .as_mut()
                .map(|o| o.as_mut_ptr())
                .unwrap_or(null_mut()),
            func_ptr,
            ret.as_mut_ptr(),
            arguments.as_mut_ptr().cast(),
            arguments.len() as u32,
            null_mut(),
        );
    })
}

/// call function with raw pointer.
/// call_fn parameters: (return_value)
pub(crate) fn call_raw_common(call_fn: impl FnOnce(&mut ZVal)) -> crate::Result<ZVal> {
    let _guard = ExceptionGuard::default();

    let mut ret = ZVal::default();

    call_fn(&mut ret);
    if ret.get_type_info().is_undef() {
        ret = ().into();
    }

    unsafe {
        if !eg!(exception).is_null() {
            let e = ptr::replace(&mut eg!(exception), null_mut());
            let obj = ZObject::from_raw(e);
            match ThrowObject::new(obj) {
                Ok(e) => return Err(e.into()),
                Err(e) => return Err(e.into()),
            }
        }
    }

    Ok(ret)
}
