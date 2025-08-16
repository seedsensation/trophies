#[macro_export]
macro_rules! into_type {
    ($i:ty, $v:expr) => {
        Into::<$i>::into($v)
    }
}

#[macro_export]
macro_rules! try_into_type {
    ($i: ty, $v: expr) => {
        TryInto::<$i>::try_into($v).unwrap()
    }
}

#[macro_export]
macro_rules! try_from_err {
    ($i:ty, $v: expr) => {
        $i::try_from($v).map_err(|_| crate::ConversionError).unwrap()
    }
}

#[macro_export]
macro_rules! try_into_err {
    ($i: ty, $v: expr) => {
        TryInto::<$i>::try_into($v).map_err(|_| crate::ConversionError).unwrap()
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
        $(println!("{}",try_into_type!(big_int::BigInt, $v as i128));)*
    }
}
