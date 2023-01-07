use proc_macro::TokenStream;
use proc_macro2::{Span};
use syn::{Token, Pat,Result,Ident, parse::{ParseStream, Parse}};
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