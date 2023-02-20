use units_macro::replace_for;

use crate::PseudoMul;

use super::*;

impl<const U: SiUnitExt, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> ops::Mul<DT>
    for Quantity<U, DT, BASE, STORAGE_BASE, true>
where
    DT: QuantityDataTraits<DT>,
{
    type Output = Self;
    fn mul(self, rhs: DT) -> Self::Output {
        Quantity {
            raw_value: self.raw_value * rhs,
        }
    }
}

impl<const U: SiUnitExt, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> ops::Mul<DT>
    for Quantity<U, DT, BASE, STORAGE_BASE, false>
where
    DT: QuantityDataTraits<DT>,
{
    type Output = Quantity<U, DT, BASE, STORAGE_BASE, true>;
    fn mul(self, rhs: DT) -> Self::Output {
        let base: Rational128 = BASE.into();
        let storage_base: Rational128 = STORAGE_BASE.into();
        // PR to Ratio????
        let storage_base_inv: Rational128 =
            Rational128::new_raw(*storage_base.denom(), *storage_base.numer());
        let product: Rational128 = base.p_mul(storage_base_inv);

        Quantity::<U, DT, BASE, STORAGE_BASE, true> {
            raw_value: { (product.p_mul(DT::ONE)) * rhs },
        }
    }
}

//type DT = f64;

// TODO: fix this to work with normal code not with stringliterals
replace_for!(f32|f64|i32, ("impl<const U: SiUnitExt, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> ops::Mul<Quantity<U, REPLACE, BASE, STORAGE_BASE>> for REPLACE where
    REPLACE: QuantityDataTraits<REPLACE>,
{
    type Output = Quantity<U, REPLACE, BASE, STORAGE_BASE, true>;

    fn mul(self, rhs: Quantity<U, REPLACE, BASE, STORAGE_BASE>) -> Self::Output {
        Quantity {
            raw_value: self * rhs.raw_value,
        }
    }

}"
));

impl<
        const UL: SiUnitExt,
        const UR: SiUnitExt,
        DT,
        const BASE: BaseUnit,
        const STORAGE_BASE: BaseUnit,
    > ops::Mul<Quantity<UR, DT, BASE, STORAGE_BASE, true>>
    for Quantity<UL, DT, BASE, STORAGE_BASE, true>
where
    Quantity<{ UL * UR }, DT, BASE, STORAGE_BASE, true>:,
    DT: QuantityDataTraits<DT>,
{
    type Output = Quantity<{ UL * UR }, DT, BASE, STORAGE_BASE, true>;

    fn mul(self, rhs: Quantity<UR, DT, BASE, STORAGE_BASE, true>) -> Self::Output {
        Quantity {
            raw_value: self.raw_value * rhs.raw_value,
        }
    }
}

impl<
        const UL: SiUnitExt,
        const UR: SiUnitExt,
        DT,
        const BASE: BaseUnit,
        const STORAGE_BASE: BaseUnit,
    > ops::Mul<Quantity<UR, DT, BASE, STORAGE_BASE, false>>
    for Quantity<UL, DT, BASE, STORAGE_BASE, false>
where
    Quantity<{ UL * UR }, DT, BASE, STORAGE_BASE, false>:,
    DT: QuantityDataTraits<DT>,
{
    type Output = Quantity<{ UL * UR }, DT, BASE, STORAGE_BASE, false>;

    fn mul(self, _rhs: Quantity<UR, DT, BASE, STORAGE_BASE, false>) -> Self::Output {
        // its ok to not calculate self / rhs because both values not being initialized indicate some kind of unit conversion
        Quantity::<{ UL * UR }, DT, BASE, STORAGE_BASE, false>::new()
    }
}
