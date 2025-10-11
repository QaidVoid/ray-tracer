use std::{
    fmt::{Display, Formatter},
    ops::{Add, Mul, Sub},
};

use crate::EPSILON;

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
        }
    }

    fn apply_op(&self, rhs: Self, op: impl Fn(f32, f32) -> f32) -> Self {
        Self::new(op(self.r, rhs.r), op(self.g, rhs.g), op(self.b, rhs.b))
    }

    pub fn to_rgb8(&self) -> [u8; 3] {
        let scale = |c: f32| (c.clamp(0., 1.) * 255.0).round() as u8;
        [scale(self.r), scale(self.g), scale(self.b)]
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply_op(rhs, |a, b| a + b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply_op(rhs, |a, b| a - b)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.apply_op(rhs, |a, b| a * b)
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.r - other.r).abs() < EPSILON
            && (self.g - other.g).abs() < EPSILON
            && (self.b - other.b).abs() < EPSILON
    }
}
