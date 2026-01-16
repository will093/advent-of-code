pub trait Integer<T> {}
pub trait Unsigned<T>: Integer<T> {}
pub trait Signed<T>: Integer<T> {}

macro_rules! impl_empty_trait {
    ($name:ident for $($t:ty)*) => ($(
        impl $name<$t> for $t {}
    )*)
}

impl_empty_trait!(Integer for u8 u16 u32 u64 u128 usize i16 i32 i64 i128);
impl_empty_trait!(Unsigned for u8 u16 u32 u64 u128 usize);
impl_empty_trait!(Signed for i16 i32 i64 i128);
