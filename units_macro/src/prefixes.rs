use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    spanned::Spanned,
    BinOp, Expr, Ident, Lit, Token,
};

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

fn exponentiate(input: &Expr) -> Result<i128> {
    match input {
        Expr::Binary(input) => {
            let mut nom_parsed: i128 = parse_expr_int(&input.left)?;
            if let BinOp::BitXor(_) = input.op {
                let exponent_parsed = parse_expr_int(&input.right)? as u32;
                nom_parsed = nom_parsed.pow(exponent_parsed);
            }
            Ok(nom_parsed)
        }

        Expr::Lit(_) => parse_expr_int(input),
        _ => Err(syn::Error::new(input.span(), "expected int")),
    }
}

fn parse_expr_int(input: &Expr) -> Result<i128> {
    if let Expr::Lit(l) = input {
        if let Lit::Int(i) = &l.lit {
            return i.base10_parse();
        }
    }
    Err(syn::Error::new(input.span(), "expected int"))
}
