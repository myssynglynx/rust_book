#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        let mut temp_fect = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    };
}
