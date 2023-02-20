#![feature(iter_intersperse)]
use itertools::Itertools;

use std::{
    collections::BTreeMap,
    fs::File,
    path::Path,
    io::Write
};

use common_serde::*;


fn main() {
    println!("cargo:rerun-if-changed=src/si/quantities_definition/");
    println!("cargo:rerun-if-changed=src/si/constants_definition/");
    println!("cargo:rerun-if-changed=src/si/prefixes/");
    println!("cargo:rerun-if-changed=types.toml");

    let path_si = Path::new("src/si");
    let path_prefixes = path_si.join("prefixes");
    let path_quantities_code = path_si.join("quantities_code");

    let cargo_toml = Path::new("types.toml");
    let storage_types = common_serde::load_storage_types(cargo_toml);

    let path_quantities_definition = Path::new("src/si/quantities_definition");
    let quantities = common_serde::load_quantities_from_dir(path_quantities_definition);
    let path_quantity_si_extended_code = path_quantities_code.join("quantities_si_extended.rs");
    let path_quantity_si_isq_code = path_quantities_code.join("quantities_si_isq.rs");

    let path_prefixes_custom_code = path_prefixes.join("custom.rs");
    let path_prefixes_si_data = path_prefixes.join("metric.toml");
    let path_prefixes_si_code = path_prefixes.join("metric.rs");
    let prefixes_si = common_serde::load_prefixes(&path_prefixes_si_data);

    generate_prefixes(&prefixes_si, &path_prefixes_si_code);

    let path_prefixes_bin_data = path_prefixes.join("bin.toml");
    let path_prefixes_bin_code = path_prefixes.join("bin.rs");
    let prefixes_bin = common_serde::load_prefixes(&path_prefixes_bin_data);

    generate_prefixes(&prefixes_bin, &path_prefixes_bin_code);

    generate_quantities(
        &quantities,
        &path_quantity_si_extended_code,
        common_serde::SystemOfQuantities::SiExtended,
    );
    // generate_quantities(
    //     &quantities,
    //     &path_quantity_si_isq_code,
    //     SystemOfQuantities::SiIsq,
    // );
    // let units_si_extended_path = path_si.join("aliases_to_units_si_extended.bin");
    // let units_si_isq_path = path_si.join("aliases_to_units_si_isq.bin");
    generate_aliases_to_units(
        quantities.clone().as_mut(),
        &path_prefixes_custom_code,
        SystemOfQuantities::SiExtended,
        &prefixes_si,
        &prefixes_bin,
    );
    // generate_units(
    //     &quantities,
    //     &units_si_isq_path,
    //     SystemOfQuantities::SiIsq,
    // );
    // let codata_src_path = Path::new("src/si/constants_definition/codata2018.txt");
    // let out_dir_path = Path::new(&out_dir);
    // //generate_codata_constants(codata_src_path, out_dir_path);
    // //generate_codata_constants(codata_src_path, out_dir_path);
}

fn generate_quantities(
    qs: &[common_serde::Quantity],
    path: &Path,
    system_of_quantities: common_serde::SystemOfQuantities,
) {
    let mut output = File::create(path).unwrap();
    writeln!(
        output,
        "
        use crate::{{prefixes::*, DisplayUnitBase, UnitSpecialization}};
        use crate::{{BaseUnit, Quantity, SiUnitExt, SiUnit}};
        use units_macro::{{generate_units_name}};
        "
    )
    .unwrap();
    for q in qs.iter() {
        let common_serde::Quantity {
            names,
            symbol,
            short_dim_formula,
            long_dim_formula,
            units_formula,
            ..
        } = q;

        let common_serde::Dimensions {
            L,
            M,
            T,
            I,
            Θ,
            N,
            J,
            A,
            ΔΘ,
            INFO,
        } = q.dimensions;
        let name = &names["en"];
        writeln!(
            output,
            "/// {name}, {short_dim_formula}, \\[{units_formula}\\];\n/// {long_dim_formula}"
        )
        .unwrap();
        match system_of_quantities {
            common_serde::SystemOfQuantities::SiIsq => {
                let temp = Θ + ΔΘ;
                writeln!(
                    output,
                    "pub const {symbol} : SiUnit = SiUnit {{{L}, {M}, {T}, {I}, {temp} , {N}, {J} , {name}}};").unwrap()
            }
            common_serde::SystemOfQuantities::SiExtended => {
                writeln!(
                        output,
                        "pub const {symbol} : SiUnit = SiUnitExt{{ {L}, {M}, {T}, {I}, {Θ} , {N}, {J}, {A},  {ΔΘ}, {INFO}, {name}}};")
            .unwrap();
            }
        }
    }
}

// fn generate_codata_constants<Storage: Float + FromStr + Display>(codata: &Path, out_dir: &Path) {
//     let codata = File::open(codata).unwrap();
//     let codata = BufReader::new(codata);
//     let storage_type = std::any::type_name::<Storage>();
//     let of = out_dir.join(format!("codata2018_{storage_type}.rs"));
//     let of = Path::new(&of);
//     let mut of = File::create(of).unwrap();

//     for line in codata.lines() {
//         let line = line.unwrap();
//         let name: String = line[..60]
//             .trim()
//             .replace(' ', "_")
//             .replace('.', "")
//             .replace('-', "_")
//             .replace([',', '(', ')'], "")
//             .replace('/', "_PER_")
//             .to_uppercase();
//         let name_pretty = line[..60]
//             .trim()
//             .replace(" mag.", " magnetic")
//             .replace(" mom.", " moment");
//         let value = line[60..85].trim().replace(' ', "_").replace("...", "");
//         let unit = line[110..].trim();

//         let quantity = quantity_from_dimension(unit);
//         if let Some(quantity) = quantity {
//             let value = if value.contains('.') {
//                 value
//             } else {
//                 format!("{value}.0")
//             };

//             let value_numeric = Storage::from_str(&value.trim().replace('_', ""));
//             if let Ok(value_numeric) = value_numeric {
//                 if !(value_numeric.is_finite() && value_numeric.is_normal()) {
//                     continue;
//                 }
//             } else {
//                 continue;
//             }

//             let unit_pretty = if unit.is_empty() {
//                 "dimensionless".to_string()
//             } else {
//                 unit.replace(' ', "⋅")
//                     .replace("^-1", "<sup>-1</sup>")
//                     .replace("^-2", "<sup>-2</sup>")
//                     .replace("^-3", "<sup>-3</sup>")
//                     .replace("^-4", "<sup>-4</sup>")
//                     .replace("^2", "<sup>2</sup>")
//                     .replace("^3", "<sup>3</sup>")
//                     .replace("^4", "<sup>4</sup>")
//             };
//             let value_pretty = if let Some((mantissa, order)) = value.split_once("_e") {
//                 format!("{mantissa}⋅10<sup>{order}</sup>")
//             } else {
//                 value.clone()
//             };

//             writeln!(
//                 of,
//                 "/// {name_pretty}, {value_pretty} \\[{unit_pretty}\\]"
//             )
//             .unwrap();
//             writeln!(
//                 of,
//                 "pub const {name}: {quantity} = {quantity}::new({value});"
//             )
//             .unwrap();
//         } else {
//             // println!("cargo:warning=Unknown unit {unit}");
//         };
//     }
// }

fn generate_aliases_to_units(
    qs: &mut [common_serde::Quantity],
    path_custom_prefixes: &Path,
    system_of_quantities: common_serde::SystemOfQuantities,
    prefixes_metric: &[common_serde::Prefix],
    prefixes_binary: &[common_serde::Prefix],
) {
    let mut output_custom_prefixes = File::create(path_custom_prefixes).unwrap();

    let map = &mut UnitsMap::new();


    for q in qs.iter() {


        let common_serde::Quantity {
            derive_metric_prefixes,
            derive_binary_prefixes,
            units,
            ..
        } = q;
        let mut units = units.clone();
        
        if let Some(unit_names) = derive_metric_prefixes {
            let units_derived_metric = derive_prefixes(prefixes_metric, unit_names, &units);
            units.append(&mut units_derived_metric.clone());
        }
        if let Some(unit_names) = derive_binary_prefixes {
            let units_derived_binary = derive_prefixes(prefixes_binary, unit_names, &units);
            units.append(&mut units_derived_binary.clone());
        }

        //now all units for this quantity are stored in the BTreeMap

        for (base_name, u) in q.units.iter() {
            let u_clone = u.clone();
            let common_serde::Unit {
                numerator,
                denominator,
                //symbol: symbol_base,
                names,
                ..
            } = u;
            
            
            //dont create a new prefix, if an prefix already exists
            let mut prefix_name = None;
            prefixes_metric.iter().chain(prefixes_binary).for_each(
                |Prefix {
                     aliases,
                     numerator,
                     denominator,
                 }| {
                    if numerator.clone() == u_clone.numerator && denominator.clone() == u_clone.denominator {
                        prefix_name = Some(aliases.clone().unwrap()[0].clone());
                    }
                 },
            );

            //sets the name of the Prefix 
            let prefix_name = 
            match prefix_name{
                Some(prefix_name) => prefix_name,
                None => {
                    let prefix_name = format!("{base_name}_prefix");
                    writeln!(
                        output_custom_prefixes,
                        "prefix!({prefix_name},{numerator},{denominator})"
                    )
                    .unwrap();
                    prefix_name
            },
            };
            
            //generates the mappings: mm -> milli, meter
            for UnitName { singular, .. } in names.clone().unwrap().values(){
                map.insert(singular.clone(), (prefix_name.clone(), base_name.clone()));
            }


        }

        
    }
    common_serde::save_units_map(map.clone());
}

fn derive_prefixes_unit(prefixes: &[common_serde::Prefix], unit: Unit) -> BTreeMap<String, Unit> {
    let to_simplify = prefixes.iter().map(|prefix| {
        (
            prefix.aliases.clone().unwrap(),
            Unit {
                numerator: (string_exponentiate(unit.numerator.clone())
                    * string_exponentiate(prefix.numerator.clone()))
                .to_string(),
                denominator: (string_exponentiate(unit.denominator.clone())
                    * string_exponentiate(prefix.denominator.clone()))
                .to_string(),
                names: Some({
                    let mut prefix_aliases = prefix.aliases.clone().unwrap();
                    let mut map = BTreeMap::new();
                    let iterator = prefix_aliases
                        .iter()
                        .map(|name_prefix| {
                            unit.names
                                .clone()
                                .unwrap()
                                .iter()
                                .map(|(lang, UnitName { singular, plural })| {
                                    (
                                        lang.clone(),
                                        UnitName {
                                            singular: format!("{name_prefix}{singular}"),
                                            plural: format!("{name_prefix}{plural}"),
                                        },
                                    )
                                })
                                .collect_vec()
                        })
                        .flatten();
                    for (key, value) in iterator.into_iter() {
                        map.insert(key, value);
                    }
                    map
                }),
                symbol: {
                    match &unit.symbol {
                        Some(symbol) => {
                            Some(format!("{}{}", prefix.aliases.clone().unwrap()[0], symbol))
                        }
                        None => None,
                    }
                },
                symbol_plural: {
                    match &unit.symbol_plural {
                        Some(symbol) => {
                            Some(format!("{}{}", prefix.aliases.clone().unwrap()[0], symbol))
                        }
                        None => None,
                    }
                },
            },
        )
    });
    let map_iter = to_simplify
        .map(|(prefix_names, unit)| {
            prefix_names
                .iter()
                .map(|current_prefix_name| (current_prefix_name.clone(), unit.clone()))
                .collect_vec()
        })
        .flatten();
    let mut map = BTreeMap::new();
    for (key, value) in map_iter {
        map.insert(key.clone(), value);
    }
    map
}

fn derive_prefixes(
    prefixes: &[common_serde::Prefix],
    units_to_suffix: &Vec<String>,
    units: &BTreeMap<String, Unit>,
) -> BTreeMap<String, Unit> {
    let mut map = BTreeMap::new();
    let units_map = units_to_suffix
        .iter()
        .map(|current_unit_id| units[current_unit_id].clone())
        .map(|current_unit| derive_prefixes_unit(prefixes, current_unit))
        .flatten();
    for (key, value) in units_map {
        map.insert(key, value);
    }
    map
}


fn generate_prefixes(prefixes: &[common_serde::Prefix], path: &Path) {
    let mut output = File::create(path).unwrap();
    writeln!(
        output,
        "use units_macro::prefix;
use crate::{{BaseUnit, Rational128}};
"
    )
    .unwrap();

    for common_serde::Prefix {
        aliases: names,
        numerator,
        denominator,
    } in prefixes.into_iter()
    {
        let names_s: String =
            Itertools::intersperse(names.clone().unwrap().iter(), &"|".to_string())
                .map(|s| s.clone())
                .collect();
        writeln!(output, "prefix!({names_s},{numerator},{denominator});").unwrap();
    }
}
