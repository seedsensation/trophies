use std::{fmt, ops::{Add, Mul, Div, Sub}, cmp::Ordering};
use crate::into_type;

const BIGINT_DECIMAL_PLACES: u32 = 3;
const MANTISSA_LENGTH: u32 = 8;


trait CanLog10 {
    fn calc_log10(&self) -> u32;
}

macro_rules! impl_log10 {
    ("int", for $($t:ty),+) => {
        $(
            impl CanLog10 for $t {
                fn calc_log10(&self) -> u32 {
                    self.ilog10()
                }
            }
        )*
    };
    ("float", for $($t:ty),+) => {
        $(
            impl CanLog10 for $t {
                fn calc_log10(&self) -> u32 {
                    self.log10() as u32
                }
            }
        )*
    }
}


impl_log10!("int", for i8, i16, i32, i64, i128);
impl_log10!("float", for f32, f64);


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
    mantissa: i128,
    exponent: i128,
}


/// Conversion from numerical types into BigInt
/// Set the exponent to log10 of the inputted number.
///
/// If the exponent is greater than the set mantissa length,
/// then cut off the every digit past the set length.
///
/// If it's less than, add 0s to the end.
///
impl<N> From<N> for BigInt
where N: Add + Sub + Mul + Div + From<u32> + From<i128> + Into<i128> + CanLog10 {
    fn from(val: N) -> BigInt {
        // exponent = log10(val)
        let exponent = val.calc_log10() as i128;

        // mantissa = first {} digits of val
        let mantissa = match exponent.cmp(&(MANTISSA_LENGTH as i128)) {
            Ordering::Greater => into_type!(val, i128) / (10i128.pow(exponent as u32 - MANTISSA_LENGTH)),
            Ordering::Equal => into_type!(val, i128),
            Ordering::Less => into_type!(val, i128) * 10i128.pow(MANTISSA_LENGTH - exponent as u32),
        };

        BigInt { exponent, mantissa }

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
            Ordering::Less => write!(f, "{}", self.mantissa / 10i128.pow(MANTISSA_LENGTH - self.exponent as u32)),
            Ordering::Greater => write!(f,"{:.4}e{}", self.mantissa as f64 / 10i128.pow(MANTISSA_LENGTH) as f64, self.exponent as u32)
        }
    }
}
