#![feature(proc_macro_diagnostic)]
#![feature(extend_one)]

use auto_unit::AutoUnit;
use auto_unit::_auto_unit;
use generate_units::{_generate_units_name, UnitsTypesDef};
//use generat_units::{UnitsTypesDef, _generate_units_name};
use impl_for::i_replace_for;
use one_trait::_impl_one;
use prefixes::{generate_unit_prefixes, PrefixesDef};
use proc_macro::TokenStream;

use syn::parse::{Parse, ParseStream, Result};

use syn::{parenthesized, parse_macro_input, Ident, LitStr, Token};

mod generate_units;
mod impl_for;
mod one_trait;
mod prefixes;
mod auto_unit;

#[derive(PartialEq, Eq, Clone, Copy)]
struct BaseUnit {
    numer: i128,
    denom: i128,
}

pub(crate) struct One(Vec<Ident>);

#[derive(Debug)]
struct Replace {
    idents: Vec<Ident>,
    tokens: String,
}

impl Parse for One {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut idents = Vec::new();
        idents.push(input.parse::<Ident>()?);
        let mut lookahead = input.lookahead1();
        while lookahead.peek(Token!(|)) {
            input.parse::<Token!(|)>()?;
            idents.push(input.parse()?);
            lookahead = input.lookahead1();
        }
        Ok(Self(idents))
    }
}

#[proc_macro]
pub fn impl_one(ts: TokenStream) -> TokenStream {
    let one = parse_macro_input!(ts as One);
    _impl_one(one)
}

#[proc_macro]
pub fn prefix(ts: TokenStream) -> TokenStream {
    let prefixes = parse_macro_input!(ts as PrefixesDef);
    generate_unit_prefixes(prefixes)
}

// impl Parse for UnitsDef {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let unit = input.parse()?;
//         input.parse::<Token!(,)>()?;

//         let names_group;
//         parenthesized!(names_group in input);

//         let names: Punctuated<Ident, Or> = names_group.parse_terminated(Ident::parse)?;
//         input.parse::<Token!(,)>()?;
//         let storage_type_name = input.parse()?;
//         input.parse::<Token!(,)>()?;
//         let storage_base = input.parse()?;
//         input.parse::<Token!(,)>()?;
//         let base = input.parse()?;
//         Ok(Self {
//             unit,
//             names: Vec::from_iter(names.iter().map(|n| {*n})),
//             //storage_type,
//             storage_type_name,
//             storage_base,
//             base,
//         })
//     }
// }

// #[proc_macro]
// pub fn generate_unit(input: TokenStream) -> TokenStream {
//     let unit_def = parse_macro_input!(input as UnitDef);
//     return _generate_unit(unit_def);
// }

// #[proc_macro]
// /// usage:
// ///     SiUnit, name1|name2|name3, Type, BaseUnit, BaseUnit
// /// Example:
// ///     length, meter_u32_mm|m_u32_mm, u32, BaseUnit::new(1,1), BaseUnit::new(1,1000)
// pub fn generate_units_explicit_name(input: TokenStream) -> TokenStream {
//     let UnitsDef {
//         unit,
//         names,
//         //storage_type,
//         storage_type_name,
//         storage_base,
//         base,
//     } = parse_macro_input!(input as UnitsDef);
//     let mut all: TokenStream = TokenStream::new();
//     for name in names {
//         let unit_def_name = UnitDef {
//             unit: unit.clone(),
//             name,
//             storage_type: storage_type_name.clone(),
//             storage_base: storage_base.clone(),
//             base: base.clone(),
//         };

//         let expanded = _generate_unit(unit_def_name);
//         <proc_macro::TokenStream as Extend<TokenStream>>::extend_one(&mut all, expanded)
//     }
//     all
// }

#[proc_macro]
/// usage:
///     SiUnit, name1|name2|name3, Type, BaseUnit, BaseUnit
/// Example:
///     length, meter__mm|m__mm, u8|u16|u32|u64, BaseUnit::new(1,1), BaseUnit::new(1,1000)
pub fn generate_units_name(input: TokenStream) -> TokenStream {
    let units_types_def = parse_macro_input!(input as UnitsTypesDef);
    _generate_units_name(units_types_def)
}

impl Parse for Replace {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut idents = Vec::new();
        idents.push(input.parse::<Ident>()?);
        let mut lookahead = input.lookahead1();
        while lookahead.peek(Token!(|)) {
            input.parse::<Token!(|)>()?;
            idents.push(input.parse()?);
            lookahead = input.lookahead1();
        }
        input.parse::<Token!(,)>()?;
        let tokenstream;
        parenthesized!(tokenstream in input);
        let tokens = tokenstream.parse::<LitStr>()?.value();
        //panic!("after the comma");
        let val = Ok(Replace { idents, tokens });
        //panic!("parsed {:?}", val);
        val
    }
}

#[proc_macro]
pub fn replace_for(input: TokenStream) -> TokenStream {
    let replace_input = parse_macro_input!(input as Replace);
    //panic!("parsing started");
    //panic!("{:#?}", replace_input);
    i_replace_for(replace_input)
}

#[proc_macro]
pub fn auto_unit(input: TokenStream) -> TokenStream{
    let auto_unit_input = parse_macro_input!(input as AutoUnit);
    _auto_unit(auto_unit_input)

}
