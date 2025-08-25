
pub struct BigInt {
   int_array: Vec<u32>,
}

impl<T> From<T> for BigInt
    where T: ToString {
    fn from(val: T) -> BigInt {
        let option_vector: Vec<Option<u32>> = val.to_string().chars().map(|x| x.to_digit(10)).collect();
        if option_vector.iter().all(|x| x.is_some()) {
            BigInt{ int_array: option_vector.iter().map(|x| x.unwrap()).collect::<Vec<_>>() }
        } else {
            let mut result: Vec<u32> = vec![];
            for i in option_vector {
                if i.is_none() {
                    break
                }
                result.push(i.unwrap());
            }
            BigInt { int_array: result }

        }
    }
}


macro_rules! bigint {
    ($a:expr) => {
        BigInt::from($a)
    };
    (vec $a:expr) => {
        BigInt::from($a).int_array
    }
}

macro_rules! vec_type {
    ($t:ty => $($x:expr),+ $(,)?) => {
        vec![$($x as $t),+]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_into_array() {
        assert_eq!(bigint!(vec 12345), vec_type!(u32 => 1,2,3,4,5));
        assert_eq!(BigInt::from(123.45).int_array, vec![1u32, 2u32, 3u32]);
        assert_eq!(BigInt::from("1a").int_array, vec![1u32])
    }
}
