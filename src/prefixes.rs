#![allow(non_upper_case_globals)]
use std::fmt::{self, Display};

use units_macro::prefix;

use crate::{BaseUnit, Rational128};

// used from uom
// pub const yotta: BaseUnit = Rational128::new_raw(10i128.pow(24), 1).into();
// pub const zetta: BaseUnit = Rational128::new_raw(10i128.pow(21), 1).into();
// pub const exa: BaseUnit = Rational128::new_raw(10i128.pow(18), 1).into();
// pub const peta: BaseUnit = Rational128::new_raw(10i128.pow(15), 1).into();
// pub const tera: BaseUnit = Rational128::new_raw(10i128.pow(12), 1).into();
// pub const giga: BaseUnit = Rational128::new_raw(10i128.pow(9), 1).into();
// pub const mega: BaseUnit = Rational128::new_raw(10i128.pow(6), 1).into();
// pub const kilo: BaseUnit = Rational128::new_raw(10i128.pow(3), 1).into();
// pub const hecto: BaseUnit = Rational128::new_raw(10i128.pow(2), 1).into();
// pub const deca: BaseUnit = Rational128::new_raw(10i128.pow(1), 1).into();
// pub const none: BaseUnit = Rational128::new_raw(1, 1).into();
// pub const deci: BaseUnit = Rational128::new_raw(1, 10i128.pow(1)).into();
// pub const centi: BaseUnit = Rational128::new_raw(1, 10i128.pow(2)).into();
// pub const milli: BaseUnit = Rational128::new_raw(1, 10i128.pow(3)).into();
// pub const micro: BaseUnit = Rational128::new_raw(1, 10i128.pow(6)).into();
// pub const nano: BaseUnit = Rational128::new_raw(1, 10i128.pow(9)).into();
// pub const pico: BaseUnit = Rational128::new_raw(1, 10i128.pow(12)).into();
// pub const femto: BaseUnit = Rational128::new_raw(1, 10i128.pow(15)).into();
// pub const atto: BaseUnit = Rational128::new_raw(1, 10i128.pow(18)).into();
// pub const zepto: BaseUnit = Rational128::new_raw(1, 10i128.pow(21)).into();
// pub const yocto: BaseUnit = Rational128::new_raw(1, 10i128.pow(24)).into();

// // Binary prefixes.into();
// pub const yobi: BaseUnit =
// Rational128::new_raw(1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024, 1).into();
// pub const zebi: BaseUnit =
// Rational128::new_raw(1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024, 1).into();
// pub const exbi: BaseUnit = Rational128::new_raw(1024 * 1024 * 1024 * 1024 * 1024 * 1024, 1).into();
// pub const pebi: BaseUnit = Rational128::new_raw(1024 * 1024 * 1024 * 1024 * 1024, 1).into();
// pub const tebi: BaseUnit = Rational128::new_raw(1024 * 1024 * 1024 * 1024, 1).into();
// pub const gibi: BaseUnit = Rational128::new_raw(1024 * 1024 * 1024, 1).into();
// pub const mebi: BaseUnit = Rational128::new_raw(1024 * 1024, 1).into();
// pub const kibi: BaseUnit = Rational128::new_raw(1024, 1).into();


//prefix!(yotta | y, 10^24 , 1);

prefix!(quetta|Q, 10^30, 1);
prefix!(ronna|R, 10^27, 1);
prefix!(yotta|Y, 10^24, 1);
prefix!(zetta|Z, 10^21, 1);
prefix!(exa|E, 10^18, 1);
prefix!(peta|P, 10^15, 1);
prefix!(tera|T, 10^12, 1);
prefix!(giga|G, 10^9, 1);
prefix!(mega|M, 10^6, 1);
prefix!(kilo|k, 10^3, 1);
prefix!(hecto|h, 10^2, 1);
prefix!(deca|da, 10^1, 1);
prefix!(none, 1, 1);
prefix!(deci|d, 1, 10^1);
prefix!(centi|c, 1, 10^2);
prefix!(milli|m, 1, 10^3);
prefix!(micro|μ, 1, 10^6);
prefix!(nano|n, 1, 10^9);
prefix!(pico|p, 1, 10^12);
prefix!(femto|f, 1, 10^15);
prefix!(atto|a, 1, 10^18);
prefix!(zepto|z, 1, 10^21);
prefix!(yocto|y, 1, 10^24);
prefix!(ronto|r, 1, 10^27);
prefix!(quecto|q, 1, 10^30);

// Binary prefixes
prefix!(yobi, 1024^8, 1);
prefix!(zebi, 1024^7, 1);
prefix!(exbi, 1024^6, 1);
prefix!(pebi, 1024^5, 1);
prefix!(tebi, 1024^4, 1);
prefix!(gibi, 1024^3, 1);
prefix!(mebi, 1024^2, 1);
prefix!(kibi, 1024^1, 1);


impl Display for BaseUnit {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == quetta {
            return write!(fmt, "quetta")
        }
        if *self == ronna {
            return write!(fmt, "ronna")
        }
        if *self == yotta {
            return write!(fmt, "yotta")
        }
        if *self == zetta {
            return write!(fmt, "zetta")
        }
        if *self == exa {
            return write!(fmt, "exa ")
        }
        if *self == peta {
            return write!(fmt, "peta")
        }
        if *self == tera {
            return write!(fmt, "tera")
        }
        if *self == giga {
            return write!(fmt, "giga")
        }
        if *self == mega {
            return write!(fmt, "mega")
        }
        if *self == kilo {
            return write!(fmt, "kilo")
        }
        if *self == hecto {
            return write!(fmt, "hecto")
        }
        if *self == deca {
            return write!(fmt, "deca")
        }
        if *self == none {
            return write!(fmt, "")
        }
        if *self == deci {
            return write!(fmt, "deci")
        }
        if *self == centi {
            return write!(fmt, "centi")
        }
        if *self == milli {
            return write!(fmt, "milli")
        }
        if *self == micro {
            return write!(fmt, "micro")
        }
        if *self == nano {
            return write!(fmt, "nano")
        }
        if *self == pico {
            return write!(fmt, "pico")
        }
        if *self == femto {
            return write!(fmt, "femto")
        }
        if *self == atto {
            return write!(fmt, "atto")
        }
        if *self == zepto {
            return write!(fmt, "zepto")
        }
        if *self == yocto {
            return write!(fmt, "yocto")
        }
        if *self == ronto {
            return write!(fmt, "ronto")
        }
        if *self == quecto {
            return write!(fmt, "quecto")
        }
        else {
            let ratio: Rational128 = (*self).into();
            return write!(fmt, "{}", ratio)
        }
    }
}
