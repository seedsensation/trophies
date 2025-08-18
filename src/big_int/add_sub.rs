use crate::BigInt;
use std::ops::{Add, Sub, Neg};
use std::cmp::Ordering;
use crate::big_int::CanConvSafely;

/// Addition of two BigInts
///
/// If the difference between both exponents > MANTISSA_LENGTH,
/// the order of magnitude is too great, so there's no point -
/// return whichever of the two is higher.
///
/// Then, shift the mantissa of `rhs.exponent` until
/// it is between 1 and 10, do the operation, then
/// shift it back to its original position.
///
/// easy, right?
impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let exponent_difference = self.exponent - rhs.exponent;
        match exponent_difference.cmp(&0) {
            // if both exponents are the same, add the mantissas, and then call construct
            Ordering::Equal => Self::reconstruct(self.mantissa + rhs.mantissa, self.exponent),
            // if rhs.exponent > self.exponent, shift self.exponent then calculate
            Ordering::Less => Self::reconstruct(rhs.mantissa + (self.mantissa / 10i128.pow(exponent_difference.abs().to_unsigned().unwrap())), rhs.exponent),
            // if self.exponent > rhs.exponent, shift rhs.exponent and then calculate
            Ordering::Greater => Self::reconstruct(self.mantissa + (rhs.mantissa / 10i128.pow(exponent_difference.abs().to_unsigned().unwrap())), self.exponent),
        }
    }
}

impl Sub for BigInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
    }
}

impl Neg for BigInt {
    type Output = Self;

    // Required method
    fn neg(self) -> Self::Output {
        let mut n_self = self;
        n_self.mantissa = -n_self.mantissa;
        return n_self;
    }
}
