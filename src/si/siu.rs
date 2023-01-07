use std::{
    fmt::{self, Display},
    ops::{Div, Mul, Neg},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct SiUnit {
    pub L: i8,  //length
    pub M: i8,  //mass
    pub T: i8,  //time
    pub I: i8,  //electrical current
    pub Θ: i8, //thermodynamic temprature
    pub N: i8,  //amount of substance
    pub J: i8,  //luminous intensity

    pub name: Option<&'static str>
}

impl const Neg for SiUnit {
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

            name: self.name
        }
    }
}

impl const Mul for SiUnit {
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

            name: self.name
        }
    }
}

impl const Div for SiUnit {
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

            name: self.name
        }
    }
}

impl Display for SiUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "l:{}, m: {}, t:{}, I:{}, Th: {}, N:{}, J:{}",
            self.L, self.M, self.T, self.I, self.Θ, self.N, self.J
        )
    }
}
