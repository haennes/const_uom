#![feature(proc_macro_diagnostic)]
#![feature(extend_one)]

//use generat_units::{UnitsTypesDef, _generate_units_name};
use impl_for::i_replace_for;
use one_trait::_impl_one;
use prefixes::{generate_unit_prefixes, PrefixesDef};
use proc_macro::TokenStream;

use syn::parse::{Parse, ParseStream, Result};

use syn::{parenthesized, parse_macro_input, Ident, LitStr, Token};

//mod generat_units;
mod impl_for;
mod one_trait;
mod prefixes;

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







//use proc_macro::TokenStream;
use proc_macro2::{Span};
use syn::{
    //Token,
    Pat,
    //Result,
    //Ident, parse::{ParseStream, Parse}
};
use quote::quote;


pub(crate) struct UnitsTypesDef {
    unit: Ident,
    names: Vec<(Ident, Option<Ident>)>,
    storage_types_names: Vec<Ident>,
    storage_base: Ident,
    base: Ident,
}

pub(crate) struct UnitDef {
    unit: Ident,
    name: Ident,
    storage_type: Ident,
    storage_base: Ident,
    base: Ident,
}

pub(crate) fn _generate_units_name(units_types_def: UnitsTypesDef) -> TokenStream {
    let all: &mut TokenStream = &mut TokenStream::new();
    for unit_def in units_types_def.into_iter() {
        let expanded = _generate_unit(unit_def);
        <proc_macro::TokenStream as Extend<TokenStream>>::extend_one(all, expanded)
    }
    all.clone()
}

pub(crate) fn _generate_unit(unit_def: UnitDef) -> TokenStream {
    let UnitDef {
        unit,
        name,
        storage_type,
        storage_base,
        base,
    } = unit_def;
    quote! {
        pub const #name: Quantity<{#unit}, #storage_type, {#base}, {#storage_base}, false> = Quantity::<{#unit}, #storage_type, {#base}, {#storage_base}, false>::new();
    }
    .into()
}

/// usage:
///     SiUnit, name1|name2|name3, Type, BaseUnit, BaseUnit
/// Example:
///     length, meter_@_mm|m_@_mm, u8|u16|u32|u64, BaseUnit::new(1,1), BaseUnit::new(1,1000)
impl Parse for UnitsTypesDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let unit = input.parse()?;
        input.parse::<Token!(,)>()?;

        // parsing of meter_@_meter | m_@_m
        let mut names = Vec::new();
        // first name
        names.push({
            let pat: Pat = input.parse()?;
            match pat {
                Pat::Ident(pat_i) => {
                    //panic!("{:?}", pat_i);
                    (
                        pat_i.ident,
                        Some({
                            match pat_i.subpat {
                                Some(subpat) => match *subpat.1 {
                                    Pat::Ident(pat_i_inner) => pat_i_inner.ident,
                                    _ => panic!("expected 2nd name"),
                                },
                                None => panic!("expected subpattern"),
                            }
                        }),
                    )
                }
                _ => {
                    panic!("expected name");
                    //return Err(syn::Error::new(input.span(), "expected int"));
                }
            }
        });
        // any other following
        let mut lookahead = input.lookahead1();
        while lookahead.peek(Token!(|)) {
            input.parse::<Token!(|)>()?;
            let pat: Pat = input.parse()?;
            names.push({
                match pat {
                    Pat::Ident(pat_i) => {
                        //panic!("{:?}", pat_i);
                        (
                            pat_i.ident,
                            Some({
                                match pat_i.subpat {
                                    Some(subpat) => match *subpat.1 {
                                        Pat::Ident(pat_i_inner) => pat_i_inner.ident,
                                        _ => panic!("expected 2nd name"),
                                    },
                                    None => panic!("expected subpattern"),
                                }
                            }),
                        )
                    }
                    _ => {
                        panic!("expected name");
                        //return Err(syn::Error::new(input.span(), "expected int"));
                    }
                }
            });
            lookahead = input.lookahead1();
        }

        input.parse::<Token!(,)>()?;

        //parsing of u8|u16|u32
        let mut storage_types_names = Vec::<Ident>::new();
        storage_types_names.push(input.parse()?);
        lookahead = input.lookahead1();
        while lookahead.peek(Token!(|)) {
            input.parse::<Token!(|)>()?;
            storage_types_names.push(input.parse()?);
            lookahead = input.lookahead1();
        }

        // parsing storage_base
        input.parse::<Token!(,)>()?;
        let storage_base = input.parse()?;

        // parsing base
        input.parse::<Token!(,)>()?;
        let base = input.parse()?;

        Ok(Self {
            unit,
            names,
            storage_types_names,
            storage_base,
            base,
        })
    }
}

impl IntoIterator for UnitsTypesDef {
    type Item = UnitDef;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let unit = self.unit;
        let storage_types_names = self.storage_types_names;
        let storage_base = self.storage_base;
        let base = self.base;
        let a: Vec<UnitDef> = self
            .names
            .iter()
            .map(|(prefix, suffix)| {
                storage_types_names.iter().map(|storage_type| {
                    if let Some(suffix) = suffix.clone() {
                        let name = Ident::new(
                            &format!(
                                "{}{}{}",
                                prefix.to_string(),
                                storage_type.to_string(),
                                suffix.to_string()
                            ),
                            Span::call_site(),
                        );
                        UnitDef {
                            unit: unit.clone(),
                            name,
                            storage_type: storage_type.clone(),
                            storage_base: storage_base.clone(),
                            base: base.clone(),
                        }
                    } else {
                        UnitDef {
                            unit: unit.clone(),
                            name: prefix.clone(),
                            storage_type: storage_type.clone(),
                            storage_base: storage_base.clone(),
                            base: base.clone(),
                        }
                    }
                })
            })
            .flatten()
            .collect();
        a.into_iter()
    }
}
