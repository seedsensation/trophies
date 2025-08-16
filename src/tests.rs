use super::*;

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
    assert_eq!(big_int!(15)
               -
               big_int!(1),
               big_int!(14))
}
