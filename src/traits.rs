use num_traits::{FromPrimitive, Num, NumAssign};

/// Required traits a number type must implement to be used with [simplify](crate::simplify).
pub trait ExtendedNumOps: Num + Clone + Copy + NumAssign + PartialOrd + FromPrimitive {}
impl<T> ExtendedNumOps for T where T: Num + Clone + Copy + NumAssign + PartialOrd + FromPrimitive {}
