use core::ops::Mul;

use num_rational::Ratio;

use crate::{ops::QuantityDataTraits, PseudoFromRational128};

pub type Rational128 = Ratio<i128>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BaseUnit {
    numer: i128,
    denom: i128,
}

impl const From<Rational128> for BaseUnit {
    fn from(rat: Rational128) -> Self {
        BaseUnit {
            numer: *rat.numer(),
            denom: *rat.denom(),
        }
    }
}

impl const From<BaseUnit> for Rational128 {
    fn from(b: BaseUnit) -> Self {
        Rational128::new_raw(b.numer, b.denom)
    }
}
#[const_trait]
pub trait PseudoMul<DT> {
    type Output;
    fn p_mul(self, rhs: DT) -> Self::Output;
}

impl<DT> const PseudoMul<DT> for Rational128
where
    DT: QuantityDataTraits<DT> + ~const Mul + ~const PseudoFromRational128,
    //                                          seems unnecessary. probably because the feature is unstable
{
    type Output = DT;
    fn p_mul(self, rhs: DT) -> Self::Output {
        let self_dt: DT = DT::from(self);
        self_dt * rhs
    }
}

impl const PseudoMul<Rational128> for Rational128 {
    type Output = Self;

    fn p_mul(self, rhs: Rational128) -> Self::Output {
        Rational128::new_raw(self.numer() * rhs.numer(), self.denom() * rhs.denom())
    }
}
