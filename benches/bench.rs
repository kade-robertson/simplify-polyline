#![feature(test)]
extern crate test;

use simplify_polyline::{simplify, Point};
use test::Bencher;

pub const BENCH_FIXTURE_1118: &str = include_str!("../fixtures/bench-1118.json");
pub const BENCH_FIXTURE_73752: &str = include_str!("../fixtures/bench-73752.json");

#[bench]
fn simplify_hq_1118_pts(b: &mut Bencher) {
    let points = serde_json::from_str::<Vec<Point>>(BENCH_FIXTURE_1118).unwrap();
    b.iter(|| simplify(&points, 1.0, true));
}

#[bench]
fn simplify_lq_1118_pts(b: &mut Bencher) {
    let points = serde_json::from_str::<Vec<Point>>(BENCH_FIXTURE_1118).unwrap();
    b.iter(|| simplify(&points, 1.0, false));
}

#[bench]
fn simplify_hq_1118_pts_tol5(b: &mut Bencher) {
    let points = serde_json::from_str::<Vec<Point>>(BENCH_FIXTURE_1118).unwrap();
    b.iter(|| simplify(&points, 5.0, true));
}

#[bench]
fn simplify_lq_1118_pts_tol5(b: &mut Bencher) {
    let points = serde_json::from_str::<Vec<Point>>(BENCH_FIXTURE_1118).unwrap();
    b.iter(|| simplify(&points, 5.0, false));
}

#[bench]
fn simplify_hq_73752_pts(b: &mut Bencher) {
    let points = serde_json::from_str::<Vec<Point>>(BENCH_FIXTURE_73752).unwrap();
    b.iter(|| simplify(&points, 1.0, true));
}

#[bench]
fn simplify_lq_73752_pts(b: &mut Bencher) {
    let points = serde_json::from_str::<Vec<Point>>(BENCH_FIXTURE_73752).unwrap();
    b.iter(|| simplify(&points, 1.0, false));
}

#[bench]
fn simplify_hq_73752_pts_tol5(b: &mut Bencher) {
    let points = serde_json::from_str::<Vec<Point>>(BENCH_FIXTURE_73752).unwrap();
    b.iter(|| simplify(&points, 5.0, true));
}

#[bench]
fn simplify_lq_73752_pts_tol5(b: &mut Bencher) {
    let points = serde_json::from_str::<Vec<Point>>(BENCH_FIXTURE_73752).unwrap();
    b.iter(|| simplify(&points, 5.0, false));
}
