use std::{fmt, ops::{Add, Mul, Div, Sub}, cmp::Ordering};
use crate::{try_into_err};

const MANTISSA_LENGTH: u32 = 8;


pub trait CanLog10 {
    fn calc_log10(&self) -> i128 ;
}

pub trait CanConvSafely {
    fn to_int(&self) -> i128;
    fn to_float(&self) -> f64;
}

macro_rules! impl_log10 {
    ("int", for $($t:ty),+) => {
        $(
            impl CanLog10 for $t {
                fn calc_log10(&self) -> i128 {
                    self.ilog10() as i128
                }
            }
        )*
    };
    ("float", for $($t:ty),+) => {
        $(
            impl CanLog10 for $t {
                fn calc_log10(&self) -> i128 {
                    self.log10() as i128
                }
            }
        )*
    }
}

macro_rules! impl_convtoint {
    (safe for $($t:ty),+) => {
        $(
            impl CanConvSafely for $t {
                fn to_int(&self) -> i128 {
                    *self as i128
                }
                fn to_float(&self) -> f64 {
                    *self as f64
                }
            }
        )*
    }
}



impl_log10!("int", for i8, i16, i32, i64, i128);
impl_log10!("float", for f32, f64);
impl_convtoint!(safe for i8, i16, i32, i64, i128, f32, f64);

/// An implementation of BigInt, for numbers with a bigger range
/// than standard 128-bit.
///
/// The mantissa stores the value of the number as an integer.
/// The mantissa must always be 4 digits, regardless of the
/// value of the number.
///
/// The exponent is the power to which it should be raised -
/// for 128, the exponent should be 2.
///
pub struct BigInt {
    pub mantissa: i128,
    pub exponent: i128,
}

#[derive(Debug)]
pub struct ConversionError;
/// Conversion from numerical types into BigInt
/// Set the exponent to log10 of the inputted number.
///
/// If the exponent is greater than the set mantissa length,
/// then cut off the every digit past the set length.
///
/// If it's less than, add 0s to the end.
///
impl BigInt {
    pub fn new<N>(val: N) -> BigInt
where N: Add + Sub + Mul + Div + TryFrom<u32> + CanLog10 + CanConvSafely,
{
        // exponent = log10(val)
        let exponent = val.calc_log10();

        // mantissa = first {} digits of val
        let mantissa = match exponent.cmp(&(MANTISSA_LENGTH as i128)) {
            Ordering::Greater => try_into_err!(i128, val) / (10i128.pow(try_into_err!(u32, exponent) - MANTISSA_LENGTH)),
            Ordering::Equal => try_into_err!(i128, val),
            Ordering::Less => (val.to_float() * 10f64.powf(MANTISSA_LENGTH as f64 - exponent as f64)).to_int(),
        };

        BigInt { exponent, mantissa }

    }

    pub fn new_from_float(val: f64) -> BigInt {
        // exponent = log10(val)
        let exponent = val.calc_log10();
        assert!(exponent >= 0);

        // mantissa = first {} digits of val
        let mantissa = match exponent.cmp(&(MANTISSA_LENGTH as i128)) {
            Ordering::Greater => val as i128 / (10i128.pow(try_into_err!(u32, exponent) - MANTISSA_LENGTH)),
            Ordering::Equal => val as i128,
            Ordering::Less => (val * 10f64.powf(MANTISSA_LENGTH as f64 - exponent as f64)) as i128,
        };

        BigInt { exponent, mantissa }

    }

    pub fn verify(&mut self) {
        while self.mantissa.calc_log10() > MANTISSA_LENGTH.into() {
            self.mantissa /= 10;
            self.exponent += 1;
        }
        while self.mantissa.calc_log10() < MANTISSA_LENGTH.into() {
            self.mantissa *= 10;
            self.exponent -= 1;
        }
    }
}


impl<N> From<N> for BigInt
where N: Add + Sub + Mul + Div + TryFrom<u32> + CanLog10 + CanConvSafely,
{
    fn from(val: N) -> BigInt {
        BigInt::new(val)
    }

}

impl CanLog10 for BigInt{
    fn calc_log10(&self) -> i128 {
        self.exponent
    }
}

impl Mul for BigInt {
    type Output = Self;
    fn mul(self, rhs: BigInt) -> Self::Output {
        let mut output = self;
        output.mantissa *= rhs.mantissa;
        output.exponent += rhs.exponent;
        output.verify();
        output
    }
}



/// Output as string
///
/// If the exponent is equal to the set mantissa length,
/// output the whole mantissa with nothing else.
///
/// If the exponent is less than the set mantissa length,
/// divide the exponent by `10^(MANTISSA_LENGTH - exponent)`,
/// and output that.
///
/// If the exponent is greater than the set mantissa length,
/// divide the mantissa (converted to a float) by mantissa_length.
/// Then, output the result, concatenated to show 4 sig fig,
/// followed by "e{`exponent`}".
///
impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.exponent.cmp(&(MANTISSA_LENGTH as i128)) {
            Ordering::Equal => write!(f, "{}", self.mantissa),
            Ordering::Less => write!(f, "{}", self.mantissa as f64 / 10f64.powf(( MANTISSA_LENGTH - self.exponent as u32 ) as f64)),
            Ordering::Greater => write!(f,"{:.4}e{}", self.mantissa as f64 / 10i128.pow(MANTISSA_LENGTH) as f64, self.exponent as u32)
        }
    }
}
