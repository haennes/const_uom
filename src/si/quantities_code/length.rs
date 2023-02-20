use super::super::*;
use crate::{prefixes::*, DisplayUnitBase, UnitSpecialization};
use crate::{BaseUnit, Quantity, SiUnitExt};
use units_macro::{generate_units_name};

pub const l: SiUnitExt = SiUnitExt { L: 1, name: Some("Length"), ..NONE };
generate_units_name!(l, meter_@_meter| m_@_m, i8| i16 | i32, none, none);
generate_units_name!(l, yottameter_@_meter, u8 | u16 | u32, yotta, yotta);


impl DisplayUnitBase for UnitSpecialization<l> {
    fn u_fmt(&self, base: BaseUnit) -> String {
        format!("{}meter", base)
    }
}

// impl
// //<DT: QuantityDataTraits<DT>, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit>
//  Display
//     for Quantity<l, u32,none, none, true>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let BASE = none;
//         let STORAGE_BASE = none;
//         let base: Rational128 = BASE.into();
//         let storage_base: Rational128 = STORAGE_BASE.into();
//         let storage_base_inv: Rational128 =
//              Rational128::new_raw(*storage_base.denom(), *storage_base.numer());
//         let product: Rational128 = base.p_mul(storage_base_inv);
//         write!(f, "{}{}", self.raw_value.p_mul(product), BASE)
//     }
// }

//let base: Rational128 = BASE.into();
// let storage_base: Rational128 = STORAGE_BASE.into();
// /// PR to Ratio????
// let storage_base_inv: Rational128 =
//     Rational128::new_raw(*storage_base.denom(), *storage_base.numer());
// let product: Rational128 = base.p_mul(storage_base_inv);

// Self {
//     raw_value: { (product.p_mul(DT::ONE)) },
// }
