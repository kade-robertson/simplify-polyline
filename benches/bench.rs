use criterion::{criterion_group, criterion_main, Criterion};

use simplify_polyline::{serde::Point2D, simplify, Point};

pub const BENCH_FIXTURE_1118: &str = include_str!("../fixtures/bench-1118.json");
pub const BENCH_FIXTURE_73752: &str = include_str!("../fixtures/bench-73752.json");

fn fixture_1118() -> Vec<Point<2, f64>> {
    let points = serde_json::from_str::<Vec<Point2D<f64>>>(BENCH_FIXTURE_1118).unwrap();
    points.into_iter().map(Point::<2, f64>::from).collect()
}

fn fixture_73752() -> Vec<Point<2, f64>> {
    let points = serde_json::from_str::<Vec<Point2D<f64>>>(BENCH_FIXTURE_73752).unwrap();
    points.into_iter().map(Point::<2, f64>::from).collect()
}

fn simplify_hq_1118_pts(c: &mut Criterion) {
    let points = fixture_1118();
    c.bench_function("simplify_hq_1118_pts", |b| {
        b.iter(|| simplify(&points, 1.0, true))
    });
}

fn simplify_lq_1118_pts(c: &mut Criterion) {
    let points = fixture_1118();
    c.bench_function("simplify_lq_1118_pts", |b| {
        b.iter(|| simplify(&points, 1.0, false))
    });
}

fn simplify_hq_1118_pts_tol5(c: &mut Criterion) {
    let points = fixture_1118();
    c.bench_function("simplify_hq_1118_pts_tol5", |b| {
        b.iter(|| simplify(&points, 5.0, true))
    });
}

fn simplify_lq_1118_pts_tol5(c: &mut Criterion) {
    let points = fixture_1118();
    c.bench_function("simplify_lq_1118_pts_tol5", |b| {
        b.iter(|| simplify(&points, 5.0, false))
    });
}

fn simplify_hq_73752_pts(c: &mut Criterion) {
    let points = fixture_73752();
    c.bench_function("simplify_hq_73752_pts", |b| {
        b.iter(|| simplify(&points, 1.0, true))
    });
}

fn simplify_lq_73752_pts(c: &mut Criterion) {
    let points = fixture_73752();
    c.bench_function("simplify_lq_73752_pts", |b| {
        b.iter(|| simplify(&points, 1.0, false))
    });
}

fn simplify_hq_73752_pts_tol5(c: &mut Criterion) {
    let points = fixture_73752();
    c.bench_function("simplify_hq_73752_pts_tol5", |b| {
        b.iter(|| simplify(&points, 5.0, true))
    });
}

fn simplify_lq_73752_pts_tol5(c: &mut Criterion) {
    let points = fixture_73752();
    c.bench_function("simplify_lq_73752_pts_tol5", |b| {
        b.iter(|| simplify(&points, 5.0, false))
    });
}

criterion_group!(
    simplify_1118,
    simplify_lq_1118_pts,
    simplify_lq_1118_pts_tol5,
    simplify_hq_1118_pts,
    simplify_hq_1118_pts_tol5
);
criterion_group!(
    simplify_73752,
    simplify_lq_73752_pts,
    simplify_lq_73752_pts_tol5,
    simplify_hq_73752_pts,
    simplify_hq_73752_pts_tol5
);

criterion_main!(simplify_1118, simplify_73752);
