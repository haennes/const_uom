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



prefix!(yotta, 10^24, 1);
prefix!(zetta, 10^21, 1);
prefix!(exa10, 10^18, 1);
prefix!(peta, 10^15, 1);
prefix!(tera, 10^12, 1);
prefix!(giga, 10^9, 1);
prefix!(mega, 10^6, 1);
prefix!(kilo, 10^3, 1);
prefix!(hecto, 10^2, 1);
prefix!(deca, 10^1, 1);
prefix!(none, 1, 1);
prefix!(deci, 1, 10^1);
prefix!(centi, 1, 10^2);
prefix!(milli, 1, 10^3);
prefix!(micro, 1, 10^6);
prefix!(nano, 1, 10^9);
prefix!(pico, 1, 10^12);
prefix!(femto, 1, 10^15);
prefix!(atto, 1, 10^18);
prefix!(zepto, 1, 10^21);
prefix!(yocto, 1, 10^24);

// // Binary prefixes.into();
// prefix!(yobi, 1024^8, 1);
// prefix!(zebi, 1024^7, 1);
// prefix!(exbi, 1024^6, 1);
// prefix!(pebi, 1024^5, 1);
// prefix!(tebi, 1024^4, 1);
// prefix!(gibi, 1024^3, 1);
// prefix!(mebi, 1024^2, 1);
// prefix!(kibi, 1024^1, 1);
// ^
//prefix!(none, 1, 1);

impl Display for BaseUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // if *self == yotta {
        //     return write!(f, "yotta");
        // }
        // if *self == yotta {
        //     return write!(f, "yotta");
        // }
        // if *self == zetta {
        //     return write!(f, "zetta");
        // }
        // if *self == exa {
        //     return write!(f, "exa ");
        // }
        // if *self == peta {
        //     return write!(f, "peta");
        // }
        // if *self == tera {
        //     return write!(f, "tera");
        // }
        // if *self == giga {
        //     return write!(f, "giga");
        // }
        // if *self == mega {
        //     return write!(f, "mega");
        // }
        // if *self == kilo {
        //     return write!(f, "kilo");
        // }
        // if *self == hecto {
        //     return write!(f, "hecto");
        // }
        // if *self == deca {
        //     return write!(f, "deca");
        // }
        // if *self == none {
        //     return write!(f, "");
        // }
        // if *self == deci {
        //     return write!(f, "deci");
        // }
        // if *self == centi {
        //     return write!(f, "centi");
        // }
        // if *self == milli {
        //     return write!(f, "milli");
        // }
        // if *self == micro {
        //     return write!(f, "micro");
        // }
        // if *self == nano {
        //     return write!(f, "nano");
        // }
        // if *self == pico {
        //     return write!(f, "pico");
        // }
        // if *self == femto {
        //     return write!(f, "femto");
        // }
        // if *self == atto {
        //     return write!(f, "atto");
        // }
        // if *self == zepto {
        //     return write!(f, "zepto");
        // }
        // if *self == yocto {
        //     return write!(f, "yocto");
        // }
        // else
        let ratio: Rational128 = (*self).into();
        write!(f, "{}", ratio)
    }
}
