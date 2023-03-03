use std::{fmt, marker::PhantomData};

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::{ExtendedNumOps, Point};

/// stub
#[derive(Serialize, Deserialize)]
pub struct Point2D<T: ExtendedNumOps> {
    x: T,
    y: T,
}

impl<T: ExtendedNumOps> From<Point2D<T>> for Point<2, T> {
    fn from(value: Point2D<T>) -> Self {
        Point {
            vec: [value.x, value.y],
        }
    }
}
