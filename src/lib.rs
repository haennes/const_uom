//! heavily based on https://git.sr.ht/~jplatte/const-unit-poc
//! upgrades the library to generic number storage-types
//! allows setting a base value
#![feature(
    adt_const_params,
    generic_const_exprs,
    doc_cfg,
    concat_idents,
    const_for,
    const_trait_impl,
    const_convert,
    trait_alias,
    trivial_bounds,
    associated_type_bounds,
    associated_type_defaults,
    const_ops,
    specialization,
    const_mut_refs,
    test
)]
// very unstable just here for the f64 test
#![feature(const_fn_floating_point_arithmetic)]
#![allow(incomplete_features, mixed_script_confusables)] // Dimension of Temperature: Θ
#![cfg_attr(feature = "non_ascii", feature(non_ascii_idents))]



mod ops;
pub mod prefixes;
//pub mod units;
//pub mod values;
pub mod si;
use crate::siu::SiUnit;
pub use si::*;

pub mod base;
pub use base::*;

pub mod traits;
pub use traits::*;

use one::One;
use ops::*;



//trait UnitSystem<S: Sized> = Neg<Output = S> + Mul<S> + Div<S> + Display;
//impl US for SiUnit {}

// impl const Display for SiUnit{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", si_fmt!(self))
//         }
// }

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
#[repr(transparent)]
pub struct Quantity<
    const U: SiUnit,
    DT: QuantityDataTraits<DT>,
    const BASE: BaseUnit,
    const STORAGE_BASE: BaseUnit, /*= BASE*/
    const INITIALIZED: bool = false,
> {
    raw_value: DT,
}

impl<
        const U: SiUnit,
        DT,
        const BASE: BaseUnit,
        const STORAGE_BASE: BaseUnit,
        const INITIALIZED: bool,
    > Quantity<U, DT, BASE, STORAGE_BASE, INITIALIZED>
where
    DT: QuantityDataTraits<DT> + ~const std::ops::Mul + ~const PseudoFromRational128,
{
    pub const fn new() -> Quantity<U, DT, BASE, STORAGE_BASE, false> {
        Quantity { raw_value: DT::ONE }
        // let base: Rational128 = BASE.into();
        // let storage_base: Rational128 = STORAGE_BASE.into();
        // /// PR to Ratio????
        // let storage_base_inv: Rational128 =
        //     Rational128::new_raw(*storage_base.denom(), *storage_base.numer());
        // let product: Rational128 = base.p_mul(storage_base_inv);

        // Self {
        //     raw_value: { (product.p_mul(DT::ONE)) },
        // }
    }

    #[allow(non_snake_case)]
    pub const fn as_DT(&self) -> DT {
        let base: Rational128 = BASE.into();
        let storage_base: Rational128 = STORAGE_BASE.into();
        let storage_base_inv: Rational128 =
            Rational128::new_raw(*storage_base.denom(), *storage_base.numer());
        let product: Rational128 = base.p_mul(storage_base_inv);
        product.p_mul(self.raw_value)
    }
}
