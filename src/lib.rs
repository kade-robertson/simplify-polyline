#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use traits::ExtendedNumOps;

/// stub
#[cfg(feature = "serde")]
pub mod serde;

mod point;
mod traits;

pub use point::Point;

fn get_sq_seg_dist<const D: usize, T: ExtendedNumOps>(
    pt: &Point<D, T>,
    start: &Point<D, T>,
    end: &Point<D, T>,
) -> T {
    let mut intersection = *start;
    let difference = end - start;

    if difference.not_origin() {
        let t = ((pt - start) * difference).value_sum() / difference.sq_dist_origin();
        if t > T::one() {
            intersection = *end;
        } else if t > T::zero() {
            intersection = intersection + (difference * t)
        }
    }

    (pt - &intersection).sq_dist_origin()
}

fn simplify_radial_dist<const D: usize, T: ExtendedNumOps>(
    points: &[Point<D, T>],
    tolerance: T,
) -> Vec<Point<D, T>> {
    let mut prev_point = points[0];
    let mut new_points = vec![prev_point];
    let mut point = prev_point;

    for pt in points.iter().skip(1) {
        point = *pt;
        if pt.sq_dist(&prev_point) > tolerance {
            new_points.push(*pt);
            prev_point = *pt;
        }
    }

    if prev_point != point {
        new_points.push(point);
    }

    new_points
}

fn simplify_dp_step<const D: usize, T: ExtendedNumOps>(
    points: &[Point<D, T>],
    first: usize,
    last: usize,
    tolerance: T,
    simplified: &mut Vec<Point<D, T>>,
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

fn simplify_douglas_peucker<const D: usize, T: ExtendedNumOps>(
    points: &[Point<D, T>],
    tolerance: T,
) -> Vec<Point<D, T>> {
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
pub fn simplify<const D: usize, T: ExtendedNumOps>(
    points: &[Point<D, T>],
    tolerance: f64,
    high_quality: bool,
) -> Vec<Point<D, T>> {
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
