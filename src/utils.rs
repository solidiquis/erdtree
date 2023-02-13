#[macro_export]
macro_rules! hash {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut hash = std::collections::HashMap::new();
            $( hash.insert($k, $v); )*
            hash
        }
    };
}
