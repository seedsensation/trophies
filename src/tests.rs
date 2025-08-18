use super::*;
use crate::big_int::CanConvSafely;

#[test]
fn test_add_bigint() {
    assert_eq!(big_int!(9,10)
               +
               big_int!(9,10),
            // =
               big_int!(1.8,11))
}


#[test]
fn test_sub_bigint() {
    assert_eq!(big_int!(73)
               -
               big_int!(7),
            // =
               big_int!(66))
}

#[test]
fn test_mul_bigint() {
    assert_eq!(big_int!(85)
               *
               big_int!(12),
            // =
               big_int!(1020)
    );
    assert_eq!(big_int!(1.5)
               *
               big_int!(2),
            // =
               big_int!(3)
    );
}

#[test]
fn test_div_bigint() {
    assert_eq!(big_int!(150)
               /
               big_int!(2),
            // =
               big_int!(75)
    );

    assert_eq!(big_int!(2395872)
               /
               big_int!(1),
            // =
               big_int!(2395872)
    );

    assert_eq!(big_int!(5,15)
               /
               big_int!(100),
            // =
               big_int!(5,13)
    );
}

#[test]
fn test_conversion() {
    println!("{} \n {}", f64::MAX, i128::MAX);
    assert!(( f64::MAX as i128 ) < i128::MAX);
    assert_eq!(big_int!(15.5).to_float().unwrap(), 15.5f64)
}
