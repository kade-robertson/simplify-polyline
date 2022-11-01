#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

pub use traits::ExtendedNumOps;

#[cfg(feature = "tests")]
use serde::{Deserialize, Serialize};

mod traits;

/// A two-diemensional point, where the value of each coordinate must implement the
/// [ExtendedNumOps] trait.
///
/// Example:
/// ```
/// use simplify_polyline::*;
///
/// let point = Point { x: 1.0, y: 1.0 };
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "tests", derive(Serialize, Deserialize))]
pub struct Point<T: ExtendedNumOps> {
    /// The x coordinate value.
    pub x: T,
    /// The y coordinate value.
    pub y: T,
}

/// Creates a [Point] struct, used for input and output for [simplify]. A type should be specified
/// as the first argument that implements the [ExtendedNumOps] trait.
///
/// Example
/// ```
/// use simplify_polyline::*;
///
/// let point = point!(1.0, 1.0);
/// ```
#[macro_export]
macro_rules! point {
    ($x:expr,$y:expr) => {
        Point { x: $x, y: $y }
    };
}

/// Creates a &[[Point]] array, used for  used for input and output for [simplify]. A type should
/// be specified as the first argument that implements the [ExtendedNumOps] trait. Point values
/// should be specified as length-2 tuples, where each value matches the input type, in (x, y)
/// order.
///
/// Example
/// ```
/// use simplify_polyline::*;
///
/// let points = points![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)];
/// ```
#[macro_export]
macro_rules! points {
    ($(($x:expr,$y:expr)),*) => {
        &[$(Point { x: $x, y: $y }),*]
    };
}

fn get_sq_dist<T: ExtendedNumOps>(p1: &Point<T>, p2: &Point<T>) -> T {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    dx * dx + dy * dy
}

fn get_sq_seg_dist<T: ExtendedNumOps>(pt: &Point<T>, start: &Point<T>, end: &Point<T>) -> T {
    let (mut x, mut y, mut dx, mut dy) = (start.x, start.y, end.x - start.x, end.y - start.y);

    if !dx.is_zero() || !dy.is_zero() {
        let t = ((pt.x - x) * dx + (pt.y - y) * dy) / (dx * dx + dy * dy);
        if t > T::one() {
            x = end.x;
            y = end.y;
        } else if t > T::zero() {
            x = x + (dx * t);
            y = y + (dy * t);
        }
    }

    dx = pt.x - x;
    dy = pt.y - y;

    dx * dx + dy * dy
}

fn simplify_radial_dist<T: ExtendedNumOps>(points: &[Point<T>], tolerance: T) -> Vec<Point<T>> {
    let mut prev_point = points[0];
    let mut new_points = vec![prev_point];
    let mut point = prev_point;

    for pt in points.iter().skip(1) {
        point = *pt;
        if get_sq_dist(pt, &prev_point) > tolerance {
            new_points.push(*pt);
            prev_point = *pt;
        }
    }

    if prev_point != point {
        new_points.push(point);
    }

    new_points
}

fn simplify_dp_step<T: ExtendedNumOps>(
    points: &[Point<T>],
    first: usize,
    last: usize,
    tolerance: T,
    simplified: &mut Vec<Point<T>>,
) {
    let mut max_sq_dist = tolerance;
    let mut max_index = 0;

    for i in first + 1..last {
        let sq_dist = get_sq_seg_dist(&points[i], &points[first], &points[last]);
        if sq_dist > max_sq_dist {
            max_index = i;
            max_sq_dist = sq_dist;
        }
    }

    if max_sq_dist > tolerance {
        if (max_index - first) > 1 {
            simplify_dp_step(points, first, max_index, tolerance, simplified);
        }
        simplified.push(points[max_index]);
        if (last - max_index) > 1 {
            simplify_dp_step(points, max_index, last, tolerance, simplified);
        }
    }
}

fn simplify_douglas_peucker<T: ExtendedNumOps>(points: &[Point<T>], tolerance: T) -> Vec<Point<T>> {
    let mut simplified = vec![points[0]];
    simplify_dp_step(points, 0, points.len() - 1, tolerance, &mut simplified);
    simplified.push(points[points.len() - 1]);

    simplified
}

/// Simplifies a polyline within a given tolerance.
///
///
/// # Arguments
///
/// - `tolerance`: A distance measurement used for both radial distance and Douglas–Peucker -- the
/// higher the tolerance, the more points will be removed from the polyline.
/// - `high_quality`: Controls the algorithm(s) to be used in simplification
///   - `true`: this will take the entire array of points and simplify using the Douglas–Peucker
///     algorithm.
///   - `false`: the list of points are first filtered using a simple radial distance algorithm,
///     and then passed to the the Douglas-Peucker algorithm for final simplification.
pub fn simplify<T: ExtendedNumOps>(
    points: &[Point<T>],
    tolerance: f64,
    high_quality: bool,
) -> Vec<Point<T>> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let tolerance_t = T::from_f64(tolerance).unwrap_or_else(T::one);

    let tolerance_sq = tolerance_t * tolerance_t;
    let intermediate = if high_quality {
        points.to_vec()
    } else {
        simplify_radial_dist(points, tolerance_sq)
    };

    simplify_douglas_peucker(&intermediate, tolerance_sq)
}
