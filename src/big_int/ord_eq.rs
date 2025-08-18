use crate::BigInt;
use std::cmp::Ordering;

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
