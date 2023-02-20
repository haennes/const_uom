use common_serde::load_units_map;
use itertools::Itertools;
use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream, Result}, parse_macro_input};

use crate::generate_units::{_generate_units_name, UnitsTypesDef};

pub(crate) struct AutoUnit {
    base: String,
    data_type: String,
    storage: String,
}

impl Parse for AutoUnit {
    fn parse(input: ParseStream) -> Result<Self> {
        let original_name = input.to_string();
        let strings: Vec<&str> = original_name.split("_").collect_vec();
        Ok(AutoUnit {
            base: strings[0].to_string(),
            data_type: strings[1].to_string(),
            storage: strings[2].to_string(),
        })
    }
}
pub(crate) fn _auto_unit(auto_unit: AutoUnit) -> TokenStream {
    let AutoUnit { base, data_type, storage, .. } = auto_unit;
    let map  = load_units_map();
    let (base_prefix, unit) = map.get(&base).unwrap();
    let (storage_prefix, _) = map.get(&storage).unwrap();
    let input = format!("{unit}, {base}_@_{storage}, {data_type}, {base_prefix}, {storage_prefix}");
    let input_tok = input.parse().unwrap();
    _generate_units_name(
        parse_macro_input!(input_tok as UnitsTypesDef)    
    )
}
