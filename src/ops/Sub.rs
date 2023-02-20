use super::*;
impl<const U: SiUnitExt, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> ops::Sub
    for Quantity<U, DT, BASE, STORAGE_BASE, true>
where
    DT: QuantityDataTraits<DT>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            raw_value: self.raw_value - rhs.raw_value,
        }
    }
}
