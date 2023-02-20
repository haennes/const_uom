use std::{
    fmt::{self, Display},
    ops::{Div, Mul, Neg},
};

pub const NONE: SiUnitExt = SiUnitExt {
    L: 0,
    M: 0,
    T: 0,
    I: 0,
    Θ: 0,
    N: 0,
    J: 0,
    A: 0,
    ΔΘ: 0,
    INFO: 0,
    name: None,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct SiUnitExt {
    pub L: i8,  //length
    pub M: i8,  //mass
    pub T: i8,  //time
    pub I: i8,  //electrical current
    pub Θ: i8, //thermodynamic temprature
    pub N: i8,  //amount of substance
    pub J: i8,  //luminous intensity
    pub A: i8,  //Angle
    pub ΔΘ: i8, //temperature interval
    pub INFO: i8, //Information
    pub name: Option<&'static str>, //example: Length
}

impl const Default for SiUnitExt {
    fn default() -> Self {
        NONE
    }
}

impl const Neg for SiUnitExt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            L: -self.L,
            M: -self.M,
            T: -self.T,
            I: -self.I,
            Θ: -self.Θ,
            N: -self.N,
            J: -self.J,
            A: -self.A,
            ΔΘ: -self.ΔΘ,
            INFO: -self.INFO,
            name: self.name,
        }
    }
}

impl const Mul for SiUnitExt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            L: self.L + rhs.L,
            M: self.M + rhs.M,
            T: self.T + rhs.T,
            I: self.I + rhs.I,
            Θ: self.Θ + rhs.Θ,
            N: self.N + rhs.N,
            J: self.J + rhs.J,
            A: self.A + rhs.A,
            ΔΘ: self.ΔΘ + rhs.ΔΘ,
            INFO: self.INFO * rhs.INFO,
            
            name: self.name,
        }
    }
}

impl const Div for SiUnitExt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            L: self.L - rhs.L,
            M: self.M - rhs.M,
            T: self.T - rhs.T,
            I: self.I - rhs.I,
            Θ: self.Θ - rhs.Θ,
            N: self.N - rhs.N,
            J: self.J - rhs.J,
            A: self.A - rhs.A,
            ΔΘ: self.ΔΘ - rhs.ΔΘ,
            INFO: self.INFO - rhs.INFO,

            name: self.name,
        }
    }
}

impl Display for SiUnitExt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "l:{}, m: {}, t:{}, I:{}, Th: {}, N:{}, J:{}, A:{}, INFO:{}",
            self.L, self.M, self.T, self.I, self.Θ, self.N, self.J, self.A, self.INFO
        )
    }
}
