pub trait Number:
    Copy
    + Clone
    + PartialEq
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! _number_impl {
    ($t:ident, $zero:literal, $one:literal) => {
        impl Number for $t {
            const ZERO: Self = $zero;
            const ONE: Self = $one;
        }
    };
}

macro_rules! _number_impl_float {
    ($t:ident) => {
        _number_impl!($t, 0.0, 1.0);
    };
}

macro_rules! _number_impl_int {
    ($t:ident) => {
        _number_impl!($t, 0, 1);
    };
}

macro_rules! _number_impl_floats {
    ($($t:ident),*) => {
        $( _number_impl_float!($t); )*
    };
}

macro_rules! _number_impl_ints {
    ($($t:ident),*) => {
        $( _number_impl_int!($t); )*
    };
}

_number_impl_floats!(f32, f64);
_number_impl_ints!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
