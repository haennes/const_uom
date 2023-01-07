use super::super::*;
use crate::{prefixes::*, DisplayUnitBase, UnitSpecialization};
use crate::{BaseUnit, Quantity, SiUnit};
use units_macro::{generate_units_name};

pub const l: SiUnit = SiUnit { L: 1, ..NONE };
//const a = {BaseUnit::new_raw(1,1)};
//const b = {BaseUnit::new_raw(1,1)};
//pub const m : Quantity<l, u16, none, none, false> = Quantity::<l, u16, none, none, false>::new();
//length, meter_u32_mm|m_u32_mm, u32, BaseUnit::new(1,1), BaseUnit::new(1,1000)
//    length, meter__mm|m__mm, u8|u16|u32|u64, BaseUnit::new(1,1), BaseUnit::new(1,1000)
generate_units_name!(l, meter_@_meter| m_@_m, i8| i16 | i32, none, none);
generate_units_name!(l, yottameter_@_meter, u8 | u16 | u32, yotta, yotta);

//units_macro::generate_unit!(a, "b");

// quasiconst! {
//     /// 1 meter
//     //#[pseudonym::alias(meter)]
//     pub const m<DT: QuantityDataTraits<DT>, const STORAGE_BASE: BaseUnit = none>:
//         Quantity<{ l },DT, {none}, STORAGE_BASE> = Quantity::<{ l }, DT, {none}, STORAGE_BASE>::new();
//     pub const meter<DT: QuantityDataTraits<DT>, const STORAGE_BASE: BaseUnit>: Quantity<{l}, DT, none, STORAGE_BASE> = Quantity::<{ l }, DT, {none}, STORAGE_BASE>::new();


//     /// 1 millimeter
//     pub const mm<DT: QuantityDataTraits<DT>, const STORAGE_BASE: BaseUnit = milli>:
//         Quantity<{ l },DT, {milli}, STORAGE_BASE> = Quantity::<{ l }, DT, {milli}, STORAGE_BASE>::new();
//     pub const millimeter<DT: QuantityDataTraits<DT>, const STORAGE_BASE: BaseUnit>:
//         Quantity<{ l },DT, {milli}, STORAGE_BASE> = Quantity::<{ l }, DT, {milli}, STORAGE_BASE>::new();


// }

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
