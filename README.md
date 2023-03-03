# simplify-polyline

[![crates.io latest version](https://img.shields.io/crates/v/simplify-polyline?style=flat-square)](https://crates.io/crates/simplify-polyline) [![crates.io total downloads](https://img.shields.io/crates/d/simplify-polyline?style=flat-square)](https://crates.io/crates/simplify-polyline) [![docs.rs status](https://img.shields.io/docsrs/simplify-polyline?style=flat-square)](https://docs.rs/simplify-polyline/latest/)

A Rust port of the Javascript [simplify-js](https://github.com/mourner/simplify-js) library.

## Examples

```rust
use simplify_polyline::*;

fn main() {
    let points = [
        Point { vec: [0.0, 0.0] }, Point { vec: [1.0, 1.0] },
        Point { vec: [2.0, 2.0] }, Point { vec: [3.0, 3.0] },
        Point { vec: [4.0, 4.0] }
    ];
    // alternatively, use the point! macro
    let points = [
        point!(0.0, 0.0), point!(1.0, 1.0), point!(2.0, 2.0),
        point!(3.0, 3.0), point!(4.0, 4.0)
    ];
    // alternatively, use the points! macro
    let points = points![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];

    // low-quality simplification (fast)
    let new_points = simplify(&points, 1.0, false);
    // low-quality simplification (slower)
    let new_points = simplify(&points, 1.0, true);
}
```

## Features

- `serde`, optional, defaults to off. Allows serializing/deserializing points.
  - Note, this only works for some dimensions, and some formats. Read the docs for more info.

## Performance

Measurements taken with an AMD Ryzen 7 5800x, in Pop!\_OS 22.04.

| Test Case                               | simplify-polyline | simplify-js |
| --------------------------------------- | ----------------: | ----------: |
| 1118 Points, Low Quality, Tolerance 1   |         16.584 μs |   52.907 μs |
| 1118 Points, High Quality, Tolerance 1  |         26.989 μs |   85.653 μs |
| 1118 Points, Low Quality, Tolerance 5   |          3.987 μs |   12.840 μs |
| 1118 Points, High Quality, Tolerance 5  |         19.497 μs |   57.901 μs |
| 73752 Points, Low Quality, Tolerance 1  |         82.251 μs |  273.075 μs |
| 73752 Points, High Quality, Tolerance 1 |       1933.700 μs | 5376.344 μs |
| 73752 Points, Low Quality, Tolerance 5  |         54.150 μs |  181.554 μs |
| 73752 Points, High Quality, Tolerance 5 |       1458.900 μs | 3921.569 μs |

## Contributing

### Tests

```shell
$ cargo test --all-features
```

or

```shell
$ cargo make test
```

### Benchmarks

```shell
$ cargo bench --all-features
```

or

```shell
$ cargo make bench
```
