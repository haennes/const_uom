use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    spanned::Spanned,
    BinOp, Expr, Ident, Lit, Token,
};
use common_serde::exponentiate;

pub struct PrefixesDef {
    aliases: Vec<Ident>,
    nom: i128,
    denom: i128,
}

struct PrefixDef {
    name: Ident,
    nom: i128,
    denom: i128,
}

pub fn generate_unit_prefixes(prefixes: PrefixesDef) -> TokenStream {
    let PrefixesDef {
        aliases,
        nom,
        denom,
    } = prefixes;
    let all: &mut TokenStream = &mut TokenStream::new();
    for name in aliases {
        let expanded = generate_unit_prefix(PrefixDef {
            name,
            nom: nom.clone(),
            denom: denom.clone(),
        });
        <proc_macro::TokenStream as Extend<TokenStream>>::extend_one(all, expanded);
    }
    all.clone()
}

fn generate_unit_prefix(prefix: PrefixDef) -> TokenStream {
    let PrefixDef { name, nom, denom } = prefix;
    quote! {
        pub const #name : BaseUnit = Rational128::new_raw(#nom, #denom).into();

    }
    .into()
}

impl Parse for PrefixesDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut aliases = Vec::<Ident>::new();
        aliases.push(input.parse()?);
        let mut lookahead = input.lookahead1();
        while lookahead.peek(Token!(|)) {
            input.parse::<Token!(|)>()?;
            let parsed = input.parse()?;
            aliases.push(parsed);
            lookahead = input.lookahead1();
        }
        input.parse::<Token!(,)>()?;
        let inputs: Punctuated<Expr, Token!(,)> = input.parse_terminated(Expr::parse)?;
        let nom = exponentiate(&inputs[0])?;
        let denom = exponentiate(&inputs[1])?;
        return Ok(Self {
            aliases,
            nom,
            denom,
        });
    }
}




