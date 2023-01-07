use units_macro::impl_one;


pub trait One {
    const ONE: Self;
}

//todo: implement this using a macro
impl_one!(u8|u16|u32|u64|u128|i8|i16|i32|i64|i128|usize|isize);

//not possible for float as it needs 1.0
impl One for f32 {
    const ONE: Self = 1.0;
}

impl One for f64 {
    const ONE: Self = 1.0;
}