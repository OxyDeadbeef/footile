// fixed.rs     Fixed-point type.
//
// Copyright (c) 2017-2018  Douglas P Lau
//
use std::fmt;
use std::ops;

/// Fixed-point type
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed {
    v: i32,
}

/// Number of bits at fixed point (16.16)
const FRACT_BITS: i32 = 16;

/// Mask of fixed fractional bits
const FRACT_MASK: i32 = ((1 << FRACT_BITS) - 1);

/// Fixed-point constants
const FX_ZERO: Fixed = Fixed { v: 0 };
const FX_EPSILON: Fixed = Fixed { v: 1 };
const FX_HALF: Fixed = Fixed {
    v: 1 << (FRACT_BITS - 1),
};
const FX_ONE: Fixed = Fixed { v: 1 << FRACT_BITS };

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", f32::from(*self))
    }
}

impl ops::Add for Fixed {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let v = self.v + other.v;
        Fixed { v }
    }
}

impl ops::Sub for Fixed {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let v = self.v - other.v;
        Fixed { v }
    }
}

impl ops::Mul for Fixed {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let v: i64 = (self.v as i64 * other.v as i64) >> FRACT_BITS;
        Fixed { v: v as i32 }
    }
}

impl ops::Div for Fixed {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let v = ((self.v as i64) << (FRACT_BITS as i64)) / other.v as i64;
        Fixed { v: v as i32 }
    }
}

impl ops::Shl<u32> for Fixed {
    type Output = Self;

    fn shl(self, other: u32) -> Self {
        let v = self.v << other;
        Fixed { v }
    }
}

impl ops::Shr<u32> for Fixed {
    type Output = Self;

    fn shr(self, other: u32) -> Self {
        let v = self.v >> other;
        Fixed { v }
    }
}

impl From<i32> for Fixed {
    /// Get a fixed point value from an i32
    fn from(i: i32) -> Self {
        let v = i << FRACT_BITS;
        Fixed { v }
    }
}

impl From<Fixed> for i32 {
    /// Get an i32 from a fixed point value
    fn from(f: Fixed) -> Self {
        f.v >> FRACT_BITS
    }
}

impl From<f32> for Fixed {
    /// Get a fixed point value from an f32
    fn from(f: f32) -> Self {
        let v = (f * (FX_ONE.v as f32)) as i32;
        Fixed { v }
    }
}

impl From<Fixed> for f32 {
    /// Get an f32 from a fixed point value
    fn from(f: Fixed) -> Self {
        f.v as f32 / FX_ONE.v as f32
    }
}

impl Fixed {
    /// Fixed value of 0.
    pub const ZERO: Fixed = FX_ZERO;

    /// Fixed value of epsilon.
    pub const EPSILON: Fixed = FX_EPSILON;

    /// Fixed value of 1/2.
    pub const HALF: Fixed = FX_HALF;

    /// Fixed value of 1.
    pub const ONE: Fixed = FX_ONE;

    /// Get the smallest value that can be represented by this type.
    pub fn min_value() -> Self {
        Fixed {
            v: i32::min_value(),
        }
    }
    /// Get the largest value that can be represented by this type.
    pub fn max_value() -> Self {
        Fixed {
            v: i32::max_value(),
        }
    }
    /// Get the absolute value of a number.
    pub fn abs(self) -> Self {
        let v = self.v.abs();
        Fixed { v }
    }
    /// Get the largest integer less than or equal to a number.
    pub fn floor(self) -> Self {
        let v = self.v & !FRACT_MASK;
        Fixed { v }
    }
    /// Get the smallest integer greater than or equal to a number.
    pub fn ceil(self) -> Self {
        (self + FX_ONE - FX_EPSILON).floor()
    }
    /// Round a number to the nearest integer.
    pub fn round(self) -> Self {
        (self + FX_HALF).floor()
    }
    /// Get the integer part of a number.
    pub fn trunc(self) -> Self {
        if self.v >= 0 {
            self.floor()
        } else {
            self.ceil()
        }
    }
    /// Get the fractional part of a number.
    pub fn fract(self) -> Self {
        let v = self.v & FRACT_MASK;
        Fixed { v }
    }
    /// Get the average of two numbers.
    pub fn avg(self, other: Fixed) -> Self {
        let v = self.v + other.v >> 1;
        Fixed { v }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp;
    #[test]
    fn test_add() {
        assert_eq!(Fixed::from(1) + Fixed::from(1), Fixed::from(2));
        assert_eq!(Fixed::from(2) + Fixed::from(2), Fixed::from(4));
        assert_eq!(Fixed::from(2) + Fixed::from(-2), Fixed::from(0));
        assert_eq!(Fixed::from(2) + Fixed::from(-4), Fixed::from(-2));
        assert_eq!(Fixed::from(1.5) + Fixed::from(1.5), Fixed::from(3));
        assert_eq!(Fixed::from(3.5) + Fixed::from(-1.25), Fixed::from(2.25));
    }
    #[test]
    fn test_sub() {
        assert_eq!(Fixed::from(1) - Fixed::from(1), Fixed::from(0));
        assert_eq!(Fixed::from(3) - Fixed::from(2), Fixed::from(1));
        assert_eq!(Fixed::from(2) - Fixed::from(-2), Fixed::from(4));
        assert_eq!(Fixed::from(2) - Fixed::from(4), Fixed::from(-2));
        assert_eq!(Fixed::from(1.5) - Fixed::from(1.5), Fixed::from(0));
        assert_eq!(Fixed::from(3.5) - Fixed::from(1.25), Fixed::from(2.25));
    }
    #[test]
    fn test_mul() {
        assert_eq!(Fixed::from(2) * Fixed::from(2), Fixed::from(4));
        assert_eq!(Fixed::from(3) * Fixed::from(-2), Fixed::from(-6));
        assert_eq!(Fixed::from(4) * Fixed::from(0.5), Fixed::from(2));
        assert_eq!(Fixed::from(-16) * Fixed::from(-16), Fixed::from(256));
        assert_eq!(Fixed::from(37) * Fixed::from(3), Fixed::from(111));
        assert_eq!(Fixed::from(128) * Fixed::from(128), Fixed::from(16384));
    }
    #[test]
    fn test_div() {
        assert_eq!(Fixed::from(4) / Fixed::from(2), Fixed::from(2));
        assert_eq!(Fixed::from(-6) / Fixed::from(2), Fixed::from(-3));
        assert_eq!(Fixed::from(2) / Fixed::from(0.5), Fixed::from(4));
        assert_eq!(Fixed::from(256) / Fixed::from(-16), Fixed::from(-16));
        assert_eq!(Fixed::from(111) / Fixed::from(3), Fixed::from(37));
        assert_eq!(Fixed::from(37) / Fixed::from(3), Fixed::from(12.33333));
        assert_eq!(Fixed::from(16384) / Fixed::from(128), Fixed::from(128));
    }
    #[test]
    fn test_shl() {
        assert_eq!(Fixed::from(0) << 2, Fixed::from(0));
        assert_eq!(Fixed::from(1) << 1, Fixed::from(2));
        assert_eq!(Fixed::from(0.5) << 1, Fixed::from(1));
        assert_eq!(Fixed::from(0.25) << 2, Fixed::from(1));
        assert_eq!(Fixed::from(0.125) << 3, Fixed::from(1));
    }
    #[test]
    fn test_shr() {
        assert_eq!(Fixed::from(0) >> 2, Fixed::from(0));
        assert_eq!(Fixed::from(1) >> 1, Fixed::from(0.5));
        assert_eq!(Fixed::from(2) >> 1, Fixed::from(1));
        assert_eq!(Fixed::from(4) >> 2, Fixed::from(1));
        assert_eq!(Fixed::from(8) >> 3, Fixed::from(1));
    }
    #[test]
    fn test_abs() {
        assert_eq!(Fixed::from(1).abs(), Fixed::from(1));
        assert_eq!(Fixed::from(500).abs(), Fixed::from(500));
        assert_eq!(Fixed::from(-500).abs(), Fixed::from(500));
        assert_eq!(Fixed::from(-1.5).abs(), Fixed::from(1.5));
        assert_eq!(Fixed::from(-2.5).abs(), Fixed::from(2.5));
    }
    #[test]
    fn test_floor() {
        assert_eq!(Fixed::from(1).floor(), Fixed::from(1));
        assert_eq!(Fixed::from(500).floor(), Fixed::from(500));
        assert_eq!(Fixed::from(1.5).floor(), Fixed::from(1));
        assert_eq!(Fixed::from(1.99999).floor(), Fixed::from(1));
        assert_eq!(Fixed::from(-0.0001).floor(), Fixed::from(-1));
        assert_eq!(Fixed::from(-2.5).floor(), Fixed::from(-3));
    }
    #[test]
    fn test_ceil() {
        assert_eq!(Fixed::from(1).ceil(), Fixed::from(1));
        assert_eq!(Fixed::from(500).ceil(), Fixed::from(500));
        assert_eq!(Fixed::from(1.5).ceil(), Fixed::from(2));
        assert_eq!(Fixed::from(1.99999).ceil(), Fixed::from(2));
        assert_eq!(Fixed::from(-0.0001).ceil(), Fixed::from(0));
        assert_eq!(Fixed::from(-2.5).ceil(), Fixed::from(-2));
    }
    #[test]
    fn test_round() {
        assert_eq!(Fixed::from(1).round(), Fixed::from(1));
        assert_eq!(Fixed::from(500).round(), Fixed::from(500));
        assert_eq!(Fixed::from(1.5).round(), Fixed::from(2));
        assert_eq!(Fixed::from(1.49999).round(), Fixed::from(1));
        assert_eq!(Fixed::from(1.99999).round(), Fixed::from(2));
        assert_eq!(Fixed::from(-0.0001).round(), Fixed::from(0));
        assert_eq!(Fixed::from(-2.5).round(), Fixed::from(-2));
        assert_eq!(Fixed::from(-2.9).round(), Fixed::from(-3));
    }
    #[test]
    fn test_trunc() {
        assert_eq!(Fixed::from(1).trunc(), Fixed::from(1));
        assert_eq!(Fixed::from(500).trunc(), Fixed::from(500));
        assert_eq!(Fixed::from(1.5).trunc(), Fixed::from(1));
        assert_eq!(Fixed::from(1.49999).trunc(), Fixed::from(1));
        assert_eq!(Fixed::from(1.99999).trunc(), Fixed::from(1));
        assert_eq!(Fixed::from(-0.0001).trunc(), Fixed::from(0));
        assert_eq!(Fixed::from(-2.5).trunc(), Fixed::from(-2));
        assert_eq!(Fixed::from(-2.9).trunc(), Fixed::from(-2));
    }
    #[test]
    fn test_fract() {
        assert_eq!(Fixed::from(0).fract(), Fixed::from(0));
        assert_eq!(Fixed::from(0.1).fract(), Fixed::from(0.1));
        assert_eq!(Fixed::from(0.9).fract(), Fixed::from(0.9));
        assert_eq!(Fixed::from(1.5).fract(), Fixed::from(0.5));
        assert_eq!(Fixed::from(-2.5).fract(), Fixed::from(0.5));
    }
    #[test]
    fn test_avg() {
        assert_eq!(Fixed::from(1).avg(Fixed::from(2)), Fixed::from(1.5));
        assert_eq!(Fixed::from(1).avg(Fixed::from(1)), Fixed::from(1));
        assert_eq!(Fixed::from(5).avg(Fixed::from(-5)), Fixed::from(0));
        assert_eq!(Fixed::from(3).avg(Fixed::from(37)), Fixed::from(20));
        assert_eq!(Fixed::from(3).avg(Fixed::from(1.5)), Fixed::from(2.25));
    }
    #[test]
    fn test_into() {
        let i: i32 = Fixed::from(37).into();
        assert_eq!(i, 37);
        let f: f32 = Fixed::from(2.5).into();
        assert_eq!(f, 2.5);
        let a: i32 = Fixed::from(2.5).into();
        assert_eq!(a, 2);
    }
    #[test]
    fn test_cmp() {
        assert!(Fixed::from(37) > Fixed::from(3));
        assert!(Fixed::from(3) < Fixed::from(37));
        assert!(Fixed::from(-4) < Fixed::from(4));
        assert_eq!(cmp::min(Fixed::from(37), Fixed::from(3)), Fixed::from(3));
        assert_eq!(cmp::max(Fixed::from(37), Fixed::from(3)), Fixed::from(37));
    }
}
