use std::str::FromStr;

use proc_macro::TokenStream;

use crate::Replace;

pub(crate) fn i_replace_for(replace: Replace) -> TokenStream{
    //panic!("b");
    let all: &mut TokenStream = &mut TokenStream::new();
    for ident in replace.idents{
        let expanded = TokenStream::from_str(&replace.tokens.replace("REPLACE", &ident.to_string())).expect("expected @replace@ in str");
        <proc_macro::TokenStream as Extend<TokenStream>>::extend_one(all, expanded);
    }
    all.clone()
}