#![allow(non_snake_case)]
mod Add;
mod AddAssign;
mod Div;
mod DivAssign;
mod Mul;
mod MulAssign;
mod Sub;
mod SubAssign;

pub use crate::BaseUnit;
use crate::One;
use crate::PseudoFromRational128;
use crate::Quantity;
pub use crate::Rational128;
use crate::SiUnit;
use std::fmt::Display;
use std::ops;

pub trait QuantityDataTraits<DT>:
    ops::Add<DT, Output = DT>
    + ops::AddAssign<DT>
    + ops::Sub<DT, Output = DT>
    + ops::SubAssign<DT>
    + ops::Div<DT, Output = DT>
    + ops::DivAssign<DT>
    + ~ const ops::Mul<DT, Output = DT> // lol this doesnt throw an error and doesnt work living on the edge ig
    + ops::MulAssign<DT>
    + One
    + ~ const PseudoFromRational128 // same here
    + Display
    + Copy // needed because of as_DT
{
}

impl<DT> QuantityDataTraits<DT> for DT where
    DT: ops::Add<DT, Output = DT>
        + ops::AddAssign<DT>
        + ops::Sub<DT, Output = DT>
        + ops::SubAssign<DT>
        + ops::Div<DT, Output = DT>
        + ops::DivAssign<DT>
        + ops::Mul<DT, Output = DT>
        //+ ~ const ops::Mul<DT, Output = DT> // lol this doesnt throw an error and doesnt work living on the edge ig
        + ops::MulAssign<DT>
        + One
        //+ ~ const PseudoFrom<Rational128> // same here
        + PseudoFromRational128
        + Display
        + Copy // needed because of as_DT
{
}

trait AsRational128 {
    fn convert(self) -> Rational128;
}

// into ....
// trait True {}
// trait False {}
// struct If<const A: bool>;
// impl True for If<true> {}
// impl False for If<false> {}
// struct IfEither<const A: bool, const B: bool>;
// impl True for IfEither<true, true> {}
// impl True for IfEither<false, true> {}
// impl True for IfEither<true, false> {}
// impl False for IfEither<false, false> {}

// struct IfZero<const a: BaseUnit>;
// impl<const a: BaseUnit> False for IfZero<a> {}
// impl True for IfZero<{ BaseUnit { denom: 0, numer: 1 } }> {}

// impl<const U: SiUnit, DT, const BASE_ONE: BaseUnit, const BASE_TWO: BaseUnit> Into<Quantity<U, DT, BASE_ONE>> for Quantity<U, DT, BASE_TWO>
// //where IfEither<{(BASE_ONE - BASE_TWO) > 0},{(BASE_ONE - BASE_TWO) < 0}>: True,
// where If<{(BASE_ONE - BASE_TWO) > 0}>: True,
// DT: QuantityDataTraits<DT>
// {
//     fn into(self) -> Quantity<U, DT, BASE_ONE> {
//         todo!()
//     }
// }
