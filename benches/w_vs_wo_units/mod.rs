
use criterion::{criterion_group};


mod add;
pub use add::*; 

mod mul;
pub use mul::*;


criterion_group!(w_vs_wo_units, w_vs_wo_units_add, w_vs_wo_units_mul);
