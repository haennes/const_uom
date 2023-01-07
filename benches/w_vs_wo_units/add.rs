
use criterion::{BenchmarkId, Criterion};
use units::{length::*, prefixes::none, Quantity};

fn add_wo_units(a: i32, b: i32) -> i32 {
    a + b
}

fn add_w_units(a: i32, b: i32) -> i32 {
    #[allow(non_upper_case_globals)]
    let a_u = m_i32_m * a;
    let b_u = m_i32_m * b;
    let res_u = a_u + b_u;
    res_u.as_DT()
}

pub fn w_vs_wo_units_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("w_vs_wo_add");
    let numbers = (0..5).map(|_i|{rand::random::<i32>()});
    for i in numbers.enumerate() {
        group.bench_with_input(BenchmarkId::new("No Units", i.0), &i, |b, i| {
            b.iter(|| add_wo_units(i.1, 2 * (i.1)))
        });

        group.bench_with_input(BenchmarkId::new("With Units", i.0), &i, |b, i| {
            b.iter(|| add_w_units(i.1, 2 * (i.1)))
        });
    }
    group.finish()
}