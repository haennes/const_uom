#![allow(non_upper_case_globals)]
pub mod base_quantities;
pub use base_quantities::*;
pub mod siu;
use siu::SiUnit;

pub const NONE: SiUnit = SiUnit {
    L: 0,
    M: 0,
    T: 0,
    I: 0,
    Θ: 0,
    N: 0,
    J: 0,

    name: None
};
