#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

use traits::ExtendedNumOps;

#[cfg(feature = "tests")]
use serde::{Deserialize, Serialize};

pub mod traits;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "tests", derive(Serialize, Deserialize))]
pub struct Point<T: ExtendedNumOps> {
    pub x: T,
    pub y: T,
}

#[macro_export]
macro_rules! point {
    ($t:ty, $x:expr,$y:expr) => {
        Point {
            x: $x as $t,
            y: $y as $t,
        }
    };
}

#[macro_export]
macro_rules! points {
    ($t: ty, $(($x:expr,$y:expr)),*) => {
        &[$(Point { x: $x as $t, y: $y as $t }),*]
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
            x += dx * t;
            y += dy * t;
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
