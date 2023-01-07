#![feature(generic_const_exprs)]
use criterion::criterion_main;
mod w_vs_wo_units;

criterion_main!(w_vs_wo_units::w_vs_wo_units,);
