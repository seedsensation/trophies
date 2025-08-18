use std::ops::{Mul, Div};
use crate::big_int::{BigInt, CanConvSafely, MANTISSA_LENGTH};
use crate::big_int;


impl Mul for BigInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // add exponents, multiply mantissa, then verify
        println!("{}", self.exponent + rhs.exponent);
        println!("{}", big_int!(5).exponent);
        //
        BigInt::reconstruct(
            ( self.mantissa * rhs.mantissa ) / 10i128.pow(MANTISSA_LENGTH),
            self.exponent + rhs.exponent
        ).verify()
    }
}

impl Div for BigInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // subtract exponents, divide mantissa, then verify
        println!("{} {}", self.mantissa, rhs.mantissa);
        println!("{}", self.mantissa / rhs.mantissa);
        BigInt::reconstruct(
            ( (self.mantissa as f64 / rhs.mantissa as f64 ) * 10f64.powf(MANTISSA_LENGTH as f64)).round().to_int().unwrap(),
            self.exponent - rhs.exponent
        ).verify()
    }
}
