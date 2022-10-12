use num_traits::{FromPrimitive, Num, NumAssign};

pub trait ExtendedNumOps: Num + Clone + Copy + NumAssign + PartialOrd + FromPrimitive {}

macro_rules! impl_extended_num_ops {
    ($T:ty) => {
        impl ExtendedNumOps for $T {}
    };
}

impl_extended_num_ops!(isize);
impl_extended_num_ops!(i8);
impl_extended_num_ops!(i16);
impl_extended_num_ops!(i32);
impl_extended_num_ops!(i64);
impl_extended_num_ops!(usize);
impl_extended_num_ops!(u8);
impl_extended_num_ops!(u16);
impl_extended_num_ops!(u32);
impl_extended_num_ops!(u64);
impl_extended_num_ops!(f32);
impl_extended_num_ops!(f64);
