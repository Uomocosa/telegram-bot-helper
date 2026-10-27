mod telegram_guard;

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

use crate::telegram_guard::TelegramGuard;

#[must_use]
#[proc_macro_attribute]
pub fn telegram_notify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fun = match syn::parse::<ItemFn>(item) {
        Ok(fun) => fun,
        Err(err) => return err.to_compile_error().into(),
    };
    TelegramGuard::insert_as_first_statement(&mut fun);
    quote!(#fun).into()
}

// no test_usage necessary
