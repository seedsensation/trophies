use std::{fmt, ops::{Add, Mul, Div, Sub}, cmp::Ordering};
use crate::{try_into_err};

const MANTISSA_LENGTH: u32 = 8;


pub trait CanLog10 {
    fn calc_log10(&self) -> i128 ;
}

pub trait CanConvSafely {
    fn to_int(&self) -> Result<i128, ConversionError>;
    fn to_float(&self) -> Result<f64, ConversionError>;
    fn to_unsigned(&self) -> Result<u32, ConversionError>;
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
                fn to_int(&self) -> Result<i128, ConversionError>{
                    let converted = *self as i128;
                    if converted >= i128::MAX {
                        return Err(ConversionError);
                    }
                    Ok(converted)
                }
                fn to_float(&self) -> Result<f64, ConversionError> {
                    let converted = *self as f64;
                    if converted >= f64::MAX {
                        return Err(ConversionError);
                    }
                    Ok(converted)
                 }
                fn to_unsigned(&self) -> Result<u32, ConversionError> {
                    let converted = *self as u32;
                    if converted >= u32::MAX {
                        return Err(ConversionError);
                    }
                    Ok(converted)
                 }
            }
        )*
    }
}



impl_log10!("int", for i8, i16, i32, i64, i128);
impl_log10!("float", for f32, f64);
impl_convtoint!(safe for i8, i16, i32, i64, i128, f32, f64, u32);

/// An implementation of BigInt, for numbers with a bigger range
/// than standard 128-bit.
///
/// The mantissa stores the value of the number as an integer.
/// The mantissa must always be 4 digits, regardless of the
/// value of the number.
///
/// The exponent is the power to which it should be raised -
/// for 128, the exponent should be 2.
#[derive(Debug, Clone, Copy)]
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
            Ordering::Greater => val.to_int().expect("Int too large to process") / (10i128.pow(exponent.to_unsigned().expect("Exponent too large to process") - MANTISSA_LENGTH)),
            Ordering::Equal => val.to_int().expect("Int too large to process"),
            Ordering::Less => (val.to_float().expect("Float too large to process") * 10f64.powf(MANTISSA_LENGTH as f64 - exponent as f64)).to_int().unwrap(),
        };

        BigInt { exponent, mantissa }

    }


    pub fn verify(&self) -> Self {
        Self::reconstruct(self.mantissa, self.exponent)
    }

    pub fn reconstruct<M,E>(mantissa: M, exponent: E) -> BigInt
    where M: CanConvSafely, E: CanConvSafely {
        Self::construct_internal(mantissa, exponent, false)
    }
    pub fn construct_new<M,E>(mantissa: M, exponent: E) -> BigInt
    where M: CanConvSafely, E: CanConvSafely {
        Self::construct_internal(mantissa, exponent, true)
    }



    fn construct_internal<M,E>(mantissa: M, exponent: E, init: bool) -> BigInt
    where M: CanConvSafely,
    E: CanConvSafely {

        let mut mantissa = mantissa.to_float().unwrap();
        let mut exponent = exponent.to_int().unwrap();
        while mantissa.calc_log10() > MANTISSA_LENGTH.into() {
            mantissa /= 10.0;
            if !init {
                exponent += 1;
            }

        }
        while mantissa.calc_log10() < MANTISSA_LENGTH.into() {
            mantissa *= 10.0;
            if !init {
                exponent -= 1;
            }
        }

        BigInt {
            mantissa: mantissa.to_int().unwrap(),
            exponent: exponent.to_int().unwrap()
        }
    }

    pub fn mantissa_as_float(&self) -> f64 {
        self.mantissa.to_float().unwrap() / MANTISSA_LENGTH.to_float().unwrap()
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

/// Ordering
///
/// If exponents are equal, sort by mantissa, otherwise sort by exponent.
impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.exponent
            .cmp(&other.exponent)
            .then(self.mantissa.cmp(&other.mantissa))
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.mantissa == other.mantissa && self.exponent == other.exponent
    }
}

impl Eq for BigInt {}



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
        println!("{} {}", self.mantissa, self.exponent);
        match self.exponent.cmp(&(MANTISSA_LENGTH as i128)) {
            Ordering::Equal => write!(f, "{}", self.mantissa),
            Ordering::Less => write!(f, "{}", self.mantissa as f64 / 10f64.powf(( MANTISSA_LENGTH - self.exponent as u32 ) as f64)),
            Ordering::Greater => write!(f,"{}e{}", self.mantissa as f64 / 10i128.pow(MANTISSA_LENGTH) as f64, self.exponent as u32)
        }
    }
}


