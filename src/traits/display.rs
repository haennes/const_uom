use std::fmt::{self, Display};

use crate::{ops::QuantityDataTraits, BaseUnit, Quantity, SiUnit};

pub trait DisplayUnitBase {
    fn u_fmt(&self, base: BaseUnit) -> String ;
}

///because you cant implement a trait for a certain value
///you have to implement it for UnitSpecialization<value> instead
pub struct UnitSpecialization<const U: SiUnit>;

default impl<const U: SiUnit> DisplayUnitBase for UnitSpecialization<U> {
    fn u_fmt(&self, base: BaseUnit) -> String {
        format!("(unit {} in base {} )",U, base)
    }
}

pub trait DisplayQuantityInitialized<const U: SiUnit, DT: QuantityDataTraits<DT>>
//where UnitSpecialization<U>: DisplayUnitBase
{
    fn q_fmt(base: BaseUnit, storage_base: BaseUnit, value: &DT) -> String;
}



impl<const U: SiUnit, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit>
    DisplayQuantityInitialized<U, DT> for Quantity<U, DT, BASE, STORAGE_BASE, true>
where
    DT: QuantityDataTraits<DT>,
    UnitSpecialization<U>: DisplayUnitBase,
{
    fn q_fmt(base: BaseUnit, storage_base: BaseUnit, value: &DT) -> String {
        format!(
            "{} {} (stored as {})",
            value,
            UnitSpecialization::<U>.u_fmt(base),
            UnitSpecialization::<U>.u_fmt(storage_base)
        )
    }
}

impl<const U: SiUnit, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> Display
    for Quantity<U, DT, BASE, STORAGE_BASE, true>
where
    DT: QuantityDataTraits<DT>,
    UnitSpecialization<U>: DisplayUnitBase,
    Quantity<U, DT, BASE, STORAGE_BASE, true>: DisplayQuantityInitialized<U, DT>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s =
            <Self as DisplayQuantityInitialized<U, DT>>::q_fmt(BASE, STORAGE_BASE, &self.raw_value);
        write!(f, "{}", s) // replace !!
    }
}

impl<const U: SiUnit, DT, const BASE: BaseUnit, const STORAGE_BASE: BaseUnit> Display
    for Quantity<U, DT, BASE, STORAGE_BASE, false>
where
    DT: QuantityDataTraits<DT>,
    UnitSpecialization<U>: DisplayUnitBase,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NOT INITIALIZED, Unit: {}, BASE: {}({}), STORAGE_BASE: {}",
            U,
            BASE,
            UnitSpecialization::<U>.u_fmt(BASE),
            UnitSpecialization::<U>.u_fmt(STORAGE_BASE)
        ) // replace !!
    }
}
