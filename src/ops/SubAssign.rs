use super::*;

impl<const U: SiUnitExt, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> ops::SubAssign
    for Quantity<U, DT, BASE, STORAGE_BASE, true>
where
    DT: QuantityDataTraits<DT>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.raw_value -= rhs.raw_value;
    }
}
