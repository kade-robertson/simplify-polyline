use std::ops::{Add, Mul, Sub};

use crate::ExtendedNumOps;

/// The basic input type for constructing and simplifying a polyline. It can have any number of components, and will
/// work with components that implement the [ExtendedNumOps] type. Dimensionality must be determined at compile time.
///
/// This also implements some basic math operations between points, like:
/// - addition
/// - subtraction
/// - component-wise multiplication
/// - scalar multiplication
///
/// ## Example
/// ```
/// use simplify_polyline::*;
///
/// let point2d: Point<2, i32> = Point { vec: [1, 1] };
/// let another_point2d: Point<2, i32> = Point { vec: [2, 2] };
///
/// assert_eq!(point2d + another_point2d, Point { vec: [3, 3] });
/// assert_eq!(point2d - another_point2d, Point { vec: [-1, -1] });
/// assert_eq!(point2d * another_point2d, Point { vec: [2, 2] });
/// assert_eq!(point2d * 7, Point { vec: [7, 7] });
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point<const D: usize, T: ExtendedNumOps> {
    /// The components of the point.
    pub vec: [T; D],
}

impl<const D: usize, T: ExtendedNumOps> Point<D, T> {
    /// Computes the squared distance between two points.
    #[inline(always)]
    pub fn sq_dist(&self, other: &Point<D, T>) -> T {
        match D {
            2 => {
                let xdist = other.vec[0] - self.vec[0];
                let ydist = other.vec[1] - self.vec[1];
                (xdist * xdist) + (ydist * ydist)
            }
            3 => {
                let xdist = other.vec[0] - self.vec[0];
                let ydist = other.vec[1] - self.vec[1];
                let zdist = other.vec[2] - self.vec[2];
                (xdist * xdist) + (ydist * ydist) + (zdist * zdist)
            }
            4 => {
                let xdist = other.vec[0] - self.vec[0];
                let ydist = other.vec[1] - self.vec[1];
                let zdist = other.vec[2] - self.vec[2];
                let wdist = other.vec[3] - self.vec[3];
                (xdist * xdist) + (ydist * ydist) + (zdist * zdist) + (wdist * wdist)
            }
            _ => self
                .vec
                .iter()
                .zip(other.vec)
                .map(|(a, b)| b - *a)
                .reduce(|acc, v| acc + (v * v))
                .unwrap_or_else(|| T::zero()),
        }
    }

    /// Computes the squared distance between this point, and the origin.
    #[inline(always)]
    pub fn sq_dist_origin(&self) -> T {
        match D {
            2 => (self.vec[0] * self.vec[0]) + (self.vec[1] * self.vec[1]),
            3 => {
                (self.vec[0] * self.vec[0])
                    + (self.vec[1] * self.vec[1])
                    + (self.vec[2] * self.vec[2])
            }
            4 => {
                (self.vec[0] * self.vec[0])
                    + (self.vec[1] * self.vec[1])
                    + (self.vec[2] * self.vec[2])
                    + (self.vec[3] * self.vec[3])
            }
            _ => self.sq_dist(&Point {
                vec: [T::zero(); D],
            }),
        }
    }

    /// Computes the sum of each value of the point.
    #[inline(always)]
    pub fn value_sum(&self) -> T {
        match D {
            2 => self.vec[0] + self.vec[1],
            3 => self.vec[0] + self.vec[1] + self.vec[2],
            4 => self.vec[0] + self.vec[1] + self.vec[2] + self.vec[3],
            _ => self
                .vec
                .into_iter()
                .reduce(|acc, v| acc + v)
                .unwrap_or_else(|| T::zero()),
        }
    }

    /// Checks if the point is the origin point (all values at zero).
    #[inline(always)]
    pub fn is_origin(&self) -> bool {
        let zero = T::zero();
        match D {
            2 => self.vec[0] == zero && self.vec[1] == zero,
            3 => self.vec[0] == zero && self.vec[1] == zero && self.vec[2] == zero,
            4 => {
                self.vec[0] == zero
                    && self.vec[1] == zero
                    && self.vec[2] == zero
                    && self.vec[3] == zero
            }
            _ => self.vec.iter().all(|v| v == &zero),
        }
    }
}

macro_rules! impl_ref_op {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a, const D: usize, T: ExtendedNumOps> $imp<$u> for &'a $t {
            type Output = $t;

            #[inline(always)]
            fn $method(self, other: $u) -> Self::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'b, const D: usize, T: ExtendedNumOps> $imp<&'b $u> for $t {
            type Output = $t;

            #[inline(always)]
            fn $method(self, other: &'b $u) -> Self::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b, const D: usize, T: ExtendedNumOps> $imp<&'b $u> for &'a $t {
            type Output = $t;

            #[inline(always)]
            fn $method(self, other: &'b $u) -> Self::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

impl<const D: usize, T: ExtendedNumOps> Add<Point<D, T>> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn add(self, rhs: Point<D, T>) -> Self::Output {
        let mut new_values = [T::zero(); D];
        match D {
            2 => {
                new_values[0] = self.vec[0] + rhs.vec[0];
                new_values[1] = self.vec[1] + rhs.vec[1];
            }
            3 => {
                new_values[0] = self.vec[0] + rhs.vec[0];
                new_values[1] = self.vec[1] + rhs.vec[1];
                new_values[2] = self.vec[2] + rhs.vec[2];
            }
            4 => {
                new_values[0] = self.vec[0] + rhs.vec[0];
                new_values[1] = self.vec[1] + rhs.vec[1];
                new_values[2] = self.vec[2] + rhs.vec[2];
                new_values[3] = self.vec[3] + rhs.vec[3];
            }
            _ => {
                let mut i = 0usize;
                while i < self.vec.len() {
                    new_values[i] = self.vec[i] + rhs.vec[i];
                    i += 1;
                }
            }
        }
        Point { vec: new_values }
    }
}
impl_ref_op!(impl Add, add for Point<D, T>, Point<D, T>);

impl<const D: usize, T: ExtendedNumOps> Sub<Point<D, T>> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn sub(self, rhs: Point<D, T>) -> Self::Output {
        match D {
            2 => {
                let mut new_values = [T::zero(); D];
                new_values[0] = self.vec[0] - rhs.vec[0];
                new_values[1] = self.vec[1] - rhs.vec[1];
                Point { vec: new_values }
            }
            _ => {
                let mut i = 0usize;
                let mut new_values = [T::zero(); D];
                while i < self.vec.len() {
                    new_values[i] = self.vec[i] - rhs.vec[i];
                    i += 1;
                }
                Point { vec: new_values }
            }
        }
    }
}
impl_ref_op!(impl Sub, sub for Point<D, T>, Point<D, T>);

impl<const D: usize, T: ExtendedNumOps> Mul<Point<D, T>> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn mul(self, rhs: Point<D, T>) -> Self::Output {
        let mut new_values = [T::zero(); D];
        match D {
            2 => {
                new_values[0] = self.vec[0] * rhs.vec[0];
                new_values[1] = self.vec[1] * rhs.vec[1];
            }
            3 => {
                new_values[0] = self.vec[0] * rhs.vec[0];
                new_values[1] = self.vec[1] * rhs.vec[1];
                new_values[2] = self.vec[2] * rhs.vec[2];
            }
            4 => {
                new_values[0] = self.vec[0] * rhs.vec[0];
                new_values[1] = self.vec[1] * rhs.vec[1];
                new_values[2] = self.vec[2] * rhs.vec[2];
                new_values[3] = self.vec[3] * rhs.vec[3];
            }
            _ => {
                let mut i = 0usize;
                while i < self.vec.len() {
                    new_values[i] = self.vec[i] * rhs.vec[i];
                    i += 1;
                }
            }
        }
        Point { vec: new_values }
    }
}
impl_ref_op!(impl Mul, mul for Point<D, T>, Point<D, T>);

impl<const D: usize, T: ExtendedNumOps> Mul<T> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        let mut new_values = [T::zero(); D];
        match D {
            2 => {
                new_values[0] = self.vec[0] * rhs;
                new_values[1] = self.vec[1] * rhs;
            }
            3 => {
                new_values[0] = self.vec[0] * rhs;
                new_values[1] = self.vec[1] * rhs;
                new_values[2] = self.vec[2] * rhs;
            }
            4 => {
                new_values[0] = self.vec[0] * rhs;
                new_values[1] = self.vec[1] * rhs;
                new_values[2] = self.vec[2] * rhs;
                new_values[3] = self.vec[3] * rhs;
            }
            _ => {
                let mut i = 0usize;
                let mut new_values = [T::zero(); D];
                while i < self.vec.len() {
                    new_values[i] = self.vec[i] * rhs;
                    i += 1;
                }
            }
        }
        Point { vec: new_values }
    }
}
impl_ref_op!(impl Mul, mul for Point<D, T>, T);

/// Creates a single [Point], where the dimension is determined from the number of values specified, and the values all
/// must implement [ExtendedNumOps] and be of the same type.
///
/// ## Example
/// ```
/// use simplify_polyline::*;
///
/// // 2d point
/// let point2d: Point<2, i32> = point!(1, 2);
/// assert_eq!(point2d.vec.len(), 2);
///
/// // 8d point
/// let point8d: Point<8, i32> = point!(1, 2, 3, 4, 5, 6, 7, 8);
/// assert_eq!(point8d.vec.len(), 8);
/// ```
#[macro_export]
macro_rules! point {
    ($($values:expr),+) => {
        Point { vec: [$($values),+] }
    };
}

/// Creates a [Point] array, where the dimension is determined by the length of the tuples in the array, and the values
/// all must implement [ExtendedNumOps] and be of the same type. Point tuples must all be the same length.
///
/// ## Example
/// ```
/// use simplify_polyline::*;
///
/// // 2d array
/// let points2d: [Point<2, f64>; 3] = points![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)];
/// assert_eq!(points2d.len(), 3);
/// assert_eq!(points2d[0].vec.len(), 2);
///
/// // 4d array
/// let points4d: [Point<4, f64>; 2] = points![(1.0, 2.0, 3.0, 4.0), (5.0, 6.0, 7.0, 8.0)];
/// assert_eq!(points4d.len(), 2);
/// assert_eq!(points4d[0].vec.len(), 4);
/// ```
#[macro_export]
macro_rules! points {
    ($(($($values:expr),+)),*) => {
        [$(Point { vec: [$($values),+] }),*]
    };
}
