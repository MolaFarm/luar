
#[macro_use]
macro_rules! vec_value {
    ($($value:expr),* $(,)?) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($value.into());
            )*
            vec
        }
    };
}