use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, BinOp, Lit, Token};
use std::fs::{self, File};
use std::str::FromStr;

use ciborium;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::path::Path;

static UNITS_MAP_PATH: Lazy<&Path> = Lazy::new(|| Path::new("../units_map.bin"));

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct Dimensions {
    pub L: i64,
    pub M: i64,
    pub T: i64,
    pub I: i64,
    pub Θ: i64,
    pub N: i64,
    pub J: i64,
    pub A: i64,
    pub ΔΘ: i64,
    pub INFO: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]

pub struct Quantity {
    pub names: BTreeMap<String, String>,
    pub symbol: String,
    pub snake_case_name: String,
    pub short_dim_formula: String,
    pub long_dim_formula: String,
    pub units_formula: String,
    ///The string is the identifier of an unit: [units.*meter*] -> meter
    pub derive_metric_prefixes: Option<Vec<String>>,
    ///The string is the identifier of an unit: [units.*byte*] -> byte
    pub derive_binary_prefixes: Option<Vec<String>>, 
    pub dimensions: Dimensions,
    pub units: BTreeMap<String, Unit>,
    ///The string is the identifier of an unit: [units.*meter*] in the case of Length
    pub reference_unit: String, 
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Unit {
    pub numerator: String,
    pub denominator: String,
    pub names: Option<BTreeMap<String, UnitName>>, ///The first String is an id of a language
    pub symbol: Option<String>, //may not exist for every Unit
    pub symbol_plural: Option<String>, //if None is equal to symbol
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UnitName {
    pub singular: String,
    pub plural: String,
}

pub enum SystemOfQuantities {
    SiIsq,
    SiExtended,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Prefix {
    pub aliases: Option<Vec<String>>, //contains all names after loading
    pub numerator: String,
    pub denominator: String,
}

/// mm -> milli, meter
pub type UnitsMap = BTreeMap<String, (String, String)>;

pub fn load_quantities_from_dir(dir: &Path) -> Vec<Quantity> {
    let mut quantities = Vec::<Quantity>::new();

    for file in fs::read_dir(dir).unwrap() {
        let path = file.unwrap().path();
        let extension = path.extension();

        if let Some(extension) = extension {
            if extension.eq("toml") {
                let q = fs::read_to_string(&path).unwrap();
                let q = toml::de::from_str(&q).unwrap();
                quantities.push(q);
            }
        }
    }

    quantities
}

pub fn load_storage_types(path: &Path) -> Vec<String> {
    #[derive(Deserialize, Serialize, Clone, PartialEq)]
    struct Types {
        types: Vec<String>,
    }
    let storage_types_s = fs::read_to_string(&path).unwrap();
    let storage_types: Types = toml::de::from_str(&storage_types_s).unwrap();

    storage_types.types
}

pub fn load_prefixes(path: &Path) -> Vec<Prefix> {
    let prefixes_s = fs::read_to_string(&path).unwrap();
    let prefixes: BTreeMap<String, Prefix> = toml::de::from_str(&prefixes_s).unwrap();
    prefixes
        .iter()
        .map(|(key, value)| {
            let mut old = value.clone();
            old.aliases = {
                let mut vec = old.aliases.unwrap_or_default();
                vec.insert(0,key.clone());
                Some(vec)
            };
            old
        })
        .collect()
}

pub fn load_units_map() -> UnitsMap {
    let units_map_path: &Path = &UNITS_MAP_PATH;
    let res: Result<UnitsMap, ciborium::de::Error<std::io::Error>> =
        ciborium::de::from_reader(File::open(units_map_path).unwrap());
    res.unwrap()
}

pub fn save_units_map(units_map: UnitsMap) {
    let units_map_path: &Path = &UNITS_MAP_PATH;
    ciborium::ser::into_writer(
        &units_map,
        File::options()
            .write(true)
            .truncate(true)
            .open(units_map_path)
            .unwrap(),
    )
    .unwrap()
}

pub fn exponentiate(input: &Expr) -> syn::parse::Result<i128> {
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

pub fn parse_expr_int(input: &Expr) -> syn::parse::Result<i128> {
    if let Expr::Lit(l) = input {
        if let Lit::Int(i) = &l.lit {
            return i.base10_parse();
        }
    }
    Err(syn::Error::new(input.span(), "expected int"))
}


pub fn string_exponentiate(input: String) -> i128{
    let ts = TokenStream::from_str(&input).unwrap();
    let parser = Punctuated::<Expr, Token!(:)>::parse_terminated;

    let e_p = parser.parse(ts.into()).unwrap();
    let e = e_p.first().unwrap();
    exponentiate(e).unwrap()

}
