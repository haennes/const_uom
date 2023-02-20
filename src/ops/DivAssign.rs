use super::*;

impl<const U: SiUnitExt, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> ops::DivAssign<DT>
    for Quantity<U, DT, BASE, STORAGE_BASE, true>
where
    DT: QuantityDataTraits<DT>,
{
    fn div_assign(&mut self, rhs: DT) {
        self.raw_value /= rhs;
    }
}
