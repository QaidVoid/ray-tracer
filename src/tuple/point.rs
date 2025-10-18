use std::ops::{Add, Deref, Sub};

use crate::tuple::{inner::Tuple, vector::Vector};

#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Point(pub Tuple);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Tuple::new(x, y, z, 1.))
    }
}

impl Deref for Point {
    type Target = Tuple;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Point> for Tuple {
    fn eq(&self, other: &Point) -> bool {
        *self == other.0
    }
}

impl PartialEq<Tuple> for Point {
    fn eq(&self, other: &Tuple) -> bool {
        self.0 == *other
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Point(self.0 + rhs.0)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
