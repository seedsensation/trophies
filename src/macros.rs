#[macro_export]
macro_rules! into_type {
    ($i:ty, $v:expr) => {
        Into::<$i>::into($v)
    }
}

#[macro_export]
macro_rules! into_types {
    ($i:ty, for $($v:expr),+) => {
        ($(Into::<$i>::into($v)),*)
    }
}


#[macro_export]
macro_rules! test_bigint {
    ($($v:expr),+) => {
        $(println!("{}",into_type!(big_int::BigInt, $v));)*
    }
}
