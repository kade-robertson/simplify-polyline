use std::ops::{Add, Mul, Sub};

use crate::ExtendedNumOps;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point<const D: usize, T: ExtendedNumOps> {
    pub vec: [T; D],
}

impl<const D: usize, T: ExtendedNumOps> Point<D, T> {
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

    #[inline(always)]
    pub fn not_origin(&self) -> bool {
        let zero = T::zero();
        match D {
            2 => self.vec[0] != zero || self.vec[1] != zero,
            3 => self.vec[0] != zero || self.vec[1] != zero || self.vec[2] != zero,
            4 => {
                self.vec[0] != zero
                    || self.vec[1] != zero
                    || self.vec[2] != zero
                    || self.vec[3] != zero
            }
            _ => self.vec.iter().any(|v| v != &zero),
        }
    }
}

impl<const D: usize, T: ExtendedNumOps> Add<&Point<D, T>> for &Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn add(self, rhs: &Point<D, T>) -> Self::Output {
        match D {
            2 => {
                let mut new_values = [T::zero(); D];
                new_values[0] = self.vec[0] + rhs.vec[0];
                new_values[1] = self.vec[1] + rhs.vec[1];
                Point { vec: new_values }
            }
            _ => {
                let mut i = 0usize;
                let mut new_values = [T::zero(); D];
                while i < self.vec.len() {
                    new_values[i] = self.vec[i] + rhs.vec[i];
                    i += 1;
                }
                Point { vec: new_values }
            }
        }
    }
}

impl<const D: usize, T: ExtendedNumOps> Add<Point<D, T>> for &Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn add(self, rhs: Point<D, T>) -> Self::Output {
        self + &rhs
    }
}

impl<const D: usize, T: ExtendedNumOps> Add<&Point<D, T>> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn add(self, rhs: &Point<D, T>) -> Self::Output {
        &self + rhs
    }
}

impl<const D: usize, T: ExtendedNumOps> Add<Point<D, T>> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn add(self, rhs: Point<D, T>) -> Self::Output {
        &self + &rhs
    }
}

impl<const D: usize, T: ExtendedNumOps> Sub<&Point<D, T>> for &Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn sub(self, rhs: &Point<D, T>) -> Self::Output {
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

impl<const D: usize, T: ExtendedNumOps> Sub<&Point<D, T>> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn sub(self, rhs: &Point<D, T>) -> Self::Output {
        &self - rhs
    }
}

impl<const D: usize, T: ExtendedNumOps> Sub<Point<D, T>> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn sub(self, rhs: Point<D, T>) -> Self::Output {
        &self - &rhs
    }
}

impl<const D: usize, T: ExtendedNumOps> Mul<&Point<D, T>> for &Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn mul(self, rhs: &Point<D, T>) -> Self::Output {
        match D {
            2 => {
                let mut new_values = [T::zero(); D];
                new_values[0] = self.vec[0] * rhs.vec[0];
                new_values[1] = self.vec[1] * rhs.vec[1];
                Point { vec: new_values }
            }
            _ => {
                let mut i = 0usize;
                let mut new_values = [T::zero(); D];
                while i < self.vec.len() {
                    new_values[i] = self.vec[i] * rhs.vec[i];
                    i += 1;
                }
                Point { vec: new_values }
            }
        }
    }
}

impl<const D: usize, T: ExtendedNumOps> Mul<Point<D, T>> for Point<D, T> {
    type Output = Point<D, T>;

    fn mul(self, rhs: Point<D, T>) -> Self::Output {
        &self * &rhs
    }
}

impl<const D: usize, T: ExtendedNumOps> Mul<T> for &Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        match D {
            2 => {
                let mut new_values = [T::zero(); D];
                new_values[0] = self.vec[0] * rhs;
                new_values[1] = self.vec[1] * rhs;
                Point { vec: new_values }
            }
            _ => {
                let mut i = 0usize;
                let mut new_values = [T::zero(); D];
                while i < self.vec.len() {
                    new_values[i] = self.vec[i] * rhs;
                    i += 1;
                }
                Point { vec: new_values }
            }
        }
    }
}

impl<const D: usize, T: ExtendedNumOps> Mul<&T> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn mul(self, rhs: &T) -> Self::Output {
        &self * *rhs
    }
}

impl<const D: usize, T: ExtendedNumOps> Mul<T> for Point<D, T> {
    type Output = Point<D, T>;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        &self * rhs
    }
}

#[macro_export]
macro_rules! point {
    ($($values:expr),+) => {
        Point { vec: [$($values),+] }
    };
}

#[macro_export]
macro_rules! points {
    ($(($($values:expr),+)),*) => {
        &[$(Point { vec: [$($values),+] }),*]
    };
}
