use super::*;

impl<const U: SiUnitExt, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> ops::Div<DT>
    for Quantity<U, DT, BASE, STORAGE_BASE, true>
where
    DT: QuantityDataTraits<DT>,
{
    type Output = Self;
    fn div(self, rhs: DT) -> Self::Output {
        Quantity {
            raw_value: self.raw_value / rhs,
        }
    }
}

// type DT = f64;
// impl<const U: SiUnit, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> ops::Div<Quantity<U, DT, BASE, STORAGE_BASE>> for DT
// where
//     Quantity<{ -U }, DT, BASE, STORAGE_BASE>:,
//     DT: QuantityDataTraits<DT>,
// {
//     type Output = Quantity<{ -U }, DT, BASE, STORAGE_BASE>;

//     fn div(self, rhs: Quantity<U, DT, BASE, STORAGE_BASE>) -> Self::Output {
//         Quantity {
//             raw_value: self / rhs.raw_value,
//         }
//     }
// }

impl<
        const UL: SiUnitExt,
        const UR: SiUnitExt,
        DT,
        const BASE: BaseUnit,
        const STORAGE_BASE: BaseUnit,
    > ops::Div<Quantity<UR, DT, BASE, STORAGE_BASE, false>>
    for Quantity<UL, DT, BASE, STORAGE_BASE, false>
where
    Quantity<{ UL / UR }, DT, BASE, STORAGE_BASE, false>:,
    DT: QuantityDataTraits<DT>,
{
    type Output = Quantity<{ UL / UR }, DT, BASE, STORAGE_BASE, false>;

    fn div(self, _rhs: Quantity<UR, DT, BASE, STORAGE_BASE, false>) -> Self::Output {
        // its ok to not calculate self / rhs because both values not being initialized indicate some kind of unit conversion
        Quantity::<{ UL / UR }, DT, BASE, STORAGE_BASE, false>::new()
    }
}

impl<
        const UL: SiUnitExt,
        const UR: SiUnitExt,
        DT,
        const BASE: BaseUnit,
        const STORAGE_BASE: BaseUnit,
    > ops::Div<Quantity<UR, DT, BASE, STORAGE_BASE, true>>
    for Quantity<UL, DT, BASE, STORAGE_BASE, false>
where
    Quantity<{ UL / UR }, DT, BASE, STORAGE_BASE, true>:,
    DT: QuantityDataTraits<DT>,
{
    type Output = Quantity<{ UL / UR }, DT, BASE, STORAGE_BASE, true>;

    fn div(self, rhs: Quantity<UR, DT, BASE, STORAGE_BASE, true>) -> Self::Output {
        Quantity {
            raw_value: self.raw_value / rhs.raw_value,
        }
    }
}

impl<
        const UL: SiUnitExt,
        const UR: SiUnitExt,
        DT,
        const BASE: BaseUnit,
        const STORAGE_BASE: BaseUnit,
    > ops::Div<Quantity<UR, DT, BASE, STORAGE_BASE, false>>
    for Quantity<UL, DT, BASE, STORAGE_BASE, true>
where
    Quantity<{ UL / UR }, DT, BASE, STORAGE_BASE, true>:,
    DT: QuantityDataTraits<DT>,
{
    type Output = Quantity<{ UL / UR }, DT, BASE, STORAGE_BASE, true>;

    fn div(self, rhs: Quantity<UR, DT, BASE, STORAGE_BASE, false>) -> Self::Output {
        Quantity {
            raw_value: self.raw_value / rhs.raw_value,
        }
    }
}
