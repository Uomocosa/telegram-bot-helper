use syn::ItemFn;

pub struct TelegramGuard;

#[rustfmt::skip]
impl TelegramGuard {
    pub(crate) fn insert_as_first_statement(fun: &mut ItemFn) { let name = fun.sig.ident.to_string(); fun.block.stmts.insert(0, syn::parse_quote! { let _telegram_guard = telegram_bot_helper::guard::Guard::from(#name); }); }
}

#[cfg(test)]
mod tests {
    use super::TelegramGuard;
    use syn::ItemFn;

    #[test]
    fn test_usage() {
        let mut fun: ItemFn = syn::parse_quote! {
            fn sample() { let body = 1; }
        };
        TelegramGuard::insert_as_first_statement(&mut fun);
        assert_eq!(fun.block.stmts.len(), 2);
        let first = fun
            .block
            .stmts
            .first()
            .map(|stmt| quote::quote!(#stmt).to_string());
        assert!(first.unwrap_or_default().contains("Guard :: from"));
    }
}
