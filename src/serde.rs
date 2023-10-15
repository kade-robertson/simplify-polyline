//! # `serde` support
//!
//! Support is somewhat limited, only 2D point types are available at the moment.
//!
//! This will eagerly parse any map-like object with an `x` and `y` property that
//! holds a deserialize-able number type, even if other keys may be present in the map.
//!
//! Much of this implementation was designed around JSON input data. If assumptions made by that don't hold for
//! your particular input and this fails when it shouldn't, open an issue.
//!
//! ## Example (JSON)
//!
//! Using `serde_json`:
//!
//! ```rust
//! use simplify_polyline::*;
//! use serde_json::from_str;
//!
//! let point_json = r#"{ "x": 5, "y": 5 }"#;
//! let point: Point<2, f64> = serde_json::from_str(point_json).unwrap();
//! ```

use crate::{ExtendedNumOps, Point};
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize,
};
use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
};

struct Point2DVisitor<T>(PhantomData<T>);

impl<'de, T: ExtendedNumOps + Deserialize<'de>> Visitor<'de> for Point2DVisitor<T> {
    type Value = Point<2, T>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "a 2-dimensional point")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut x_value: Option<T> = None;
        let mut y_value: Option<T> = None;

        loop {
            if let Ok(maybe_entry) = map.next_entry::<String, T>() {
                if let Some((key, value)) = maybe_entry {
                    match key.as_str() {
                        "x" => x_value = Some(value),
                        "y" => y_value = Some(value),
                        _ => {}
                    }
                } else {
                    break;
                }
            }
        }

        match (x_value, y_value) {
            (Some(x), Some(y)) => Ok(Point { vec: [x, y] }),
            (None, Some(_)) => Err(de::Error::missing_field("x")),
            (Some(_), None) => Err(de::Error::missing_field("y")),
            _ => Err(de::Error::missing_field("x AND y")),
        }
    }
}

impl<'de, T: ExtendedNumOps + Deserialize<'de>> Deserialize<'de> for Point<2, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(Point2DVisitor(PhantomData))
    }
}

struct Point3DVisitor<T>(PhantomData<T>);

impl<'de, T: ExtendedNumOps + Deserialize<'de>> Visitor<'de> for Point3DVisitor<T> {
    type Value = Point<3, T>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "a 3-dimensional point")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut x_value: Option<T> = None;
        let mut y_value: Option<T> = None;
        let mut z_value: Option<T> = None;

        loop {
            if let Ok(maybe_entry) = map.next_entry::<String, T>() {
                if let Some((key, value)) = maybe_entry {
                    match key.as_str() {
                        "x" => x_value = Some(value),
                        "y" => y_value = Some(value),
                        "z" => z_value = Some(value),
                        _ => {}
                    }
                } else {
                    break;
                }
            }
        }

        match (x_value, y_value, z_value) {
            (Some(x), Some(y), Some(z)) => Ok(Point { vec: [x, y, z] }),
            (None, Some(_), Some(_)) => Err(de::Error::missing_field("x")),
            (Some(_), None, Some(_)) => Err(de::Error::missing_field("y")),
            (Some(_), Some(_), None) => Err(de::Error::missing_field("z")),
            (None, None, Some(_)) => Err(de::Error::missing_field("x AND y")),
            (None, Some(_), None) => Err(de::Error::missing_field("x AND z")),
            (Some(_), None, None) => Err(de::Error::missing_field("y AND z")),
            _ => Err(de::Error::missing_field("x AND y AND z")),
        }
    }
}

impl<'de, T: ExtendedNumOps + Deserialize<'de>> Deserialize<'de> for Point<3, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(Point3DVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;
    use serde_json::{from_value, json};

    mod point2d {
        use super::*;

        #[test]
        fn err_map_no_x_or_y() {
            let json_data = json!({ "other": "properties", "even_nums": 9 });
            let point = from_value::<Point<2, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn err_map_no_x() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "y": 5 });
            let point = from_value::<Point<2, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn err_map_no_y() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "x": 5 });
            let point = from_value::<Point<2, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn ok_x_and_y() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "x": 5, "y": 5 });
            let point = from_value::<Point<2, f64>>(json_data);
            assert!(point.is_ok());
        }
    }

    mod point3d {
        use super::*;

        #[test]
        fn err_map_no_x_or_y_or_z() {
            let json_data = json!({ "other": "properties", "even_nums": 9 });
            let point = from_value::<Point<3, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn err_map_no_x_or_y() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "z": 5 });
            let point = from_value::<Point<3, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn err_map_no_x_or_z() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "y": 5 });
            let point = from_value::<Point<3, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn err_map_no_y_or_z() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "x": 5 });
            let point = from_value::<Point<3, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn err_map_no_x() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "y": 5, "z": 5 });
            let point = from_value::<Point<3, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn err_map_no_y() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "x": 5, "z": 5 });
            let point = from_value::<Point<3, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn err_map_no_z() {
            let json_data = json!({ "other": "properties", "even_nums": 9, "x": 5, "y": 5 });
            let point = from_value::<Point<3, f64>>(json_data);
            assert!(point.is_err());
        }

        #[test]
        fn ok_x_and_y_and_z() {
            let json_data =
                json!({ "other": "properties", "even_nums": 9, "x": 5, "y": 5, "z": 5 });
            let point = from_value::<Point<3, f64>>(json_data);
            assert!(point.is_ok());
        }
    }
}
