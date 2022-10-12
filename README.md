# simplify-polyline

[![crates.io total downloads](https://img.shields.io/crates/d/simplify-polyline?style=flat-square)](https://crates.io/crates/simplify-polyline) [![docs.rs status](https://img.shields.io/docsrs/simplify-polyline?style=flat-square)](https://docs.rs/simplify-polyline/latest/)

A nearly line-by-line Rust port of the Javascript [simplify-js](https://github.com/mourner/simplify-js) library.

## Examples

```rust
use simplify_polyline::*;

fn main() {
    let points = &[
        Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 },
        Point { x: 2.0, y: 2.0 }, Point { x: 3.0, y: 3.0 },
        Point { x: 4.0, y: 4.0 }
    ];
    // alternatively, use the point! macro
    let points = &[
        point!(f64, 0.0, 0.0), point!(f64, 1.0, 1.0), point!(f64, 2.0, 2.0),
        point!(f64, 3.0, 3.0), point!(f64, 4.0, 4.0)
    ];
    // alternatively, use the points! macro
    let points = points![f64, (0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];

    // low-quality simplification (fast)
    let new_points = simplify(points, 1, false);
    // low-quality simplification (slower)
    let new_points = simplify(points, 1, true);
}
```

## Performance

Measurements taken with an AMD Ryzen 7 5800x, in Pop!\_OS 22.04.

| Test Case                               | simplify-polyline | simplify-js |
| --------------------------------------- | ----------------: | ----------: |
| 1118 Points, Low Quality, Tolerance 1   |         16.584 μs |   52.907 μs |
| 1118 Points, High Quality, Tolerance 1  |         30.910 μs |   85.653 μs |
| 1118 Points, Low Quality, Tolerance 5   |          3.987 μs |   12.840 μs |
| 1118 Points, High Quality, Tolerance 5  |         23.215 μs |   57.901 μs |
| 73752 Points, Low Quality, Tolerance 1  |         82.251 μs |  273.075 μs |
| 73752 Points, High Quality, Tolerance 1 |       1974.004 μs | 5376.344 μs |
| 73752 Points, Low Quality, Tolerance 5  |         54.150 μs |  181.554 μs |
| 73752 Points, High Quality, Tolerance 5 |       1460.742 μs | 3921.569 μs |

## Contributing

Running tests:

```shell
$ cargo test --features tests
```

Running benchmarks:

```shell
$ cargo +nightly bench --features tests
```
