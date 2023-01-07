use crate::Rational128;

#[const_trait]
pub trait PseudoFromRational128 {
    fn from(rat: Rational128) -> Self;
}

impl const PseudoFromRational128 for u8 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as u8
    }
}

impl const PseudoFromRational128 for i8 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as i8
    }
}

impl const PseudoFromRational128 for u16 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as u16
    }
}

impl const PseudoFromRational128 for i16 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as i16
    }
}

impl const PseudoFromRational128 for u32 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as u32
    }
}

impl const PseudoFromRational128 for i32 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as i32
    }
}

impl const PseudoFromRational128 for u64 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as u64
    }
}

impl const PseudoFromRational128 for i64 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as i64
    }
}

impl const PseudoFromRational128 for u128 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as u128
    }
}

impl const PseudoFromRational128 for i128 {
    // assumes rat = DT::ONE
    fn from(rat: Rational128) -> Self {
        ((*rat.numer() as u128) / (*rat.denom() as u128)) as i128
    }
}
