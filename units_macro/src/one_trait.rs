use proc_macro::TokenStream;
use quote::quote;
use crate::One;

pub(crate) fn _impl_one(types: One) -> TokenStream{
    let all: &mut TokenStream = &mut TokenStream::new();
    for ty in types.0{
        let expanded = quote!{
            impl One for #ty{
                const ONE: #ty = 1;
            }
        }.into();
        <proc_macro::TokenStream as Extend<TokenStream>>::extend_one(all, expanded);
    }
    all.clone()
    
}