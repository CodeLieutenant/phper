use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::Ident;
use syn::{parse_macro_input, Token};

struct ZendFn {
    original_zend_fn: Ident,
    const_name: Ident,
}

impl Parse for ZendFn {
    fn parse(input: &'_ ParseBuffer<'_>) -> syn::Result<Self> {
        let original_zend_fn: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let const_name: Ident = input.parse()?;
        Ok(ZendFn {
            original_zend_fn,
            const_name,
        })
    }
}

pub(crate) fn zend_create_fn(input: TokenStream) -> TokenStream {
    let ZendFn {
        original_zend_fn,
        const_name,
    } = parse_macro_input!(input as ZendFn);

    let fn_name = format!("zend_create_fn_{}", original_zend_fn);
    let new_fn_name = Ident::new(&fn_name, Span::call_site());

    let result = quote! {
        unsafe extern "C" fn #new_fn_name() -> *mut phper::sys::zend_class_entry
        {
            unsafe { #original_zend_fn() as *mut phper::sys::zend_class_entry }
        }

        pub(crate) const #const_name: unsafe extern "C" fn() -> *mut phper::sys::zend_class_entry = #new_fn_name;
    };

    result.into()
}
