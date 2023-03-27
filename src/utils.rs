use std::{borrow::ToOwned, cmp::Eq, collections::HashSet, hash::Hash};

#[macro_export]
/// Ruby-like way to crate a hashmap.
macro_rules! hash {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut hash = std::collections::HashMap::new();
            $( hash.insert($k, $v); )*
            hash
        }
    };
}

/// Ensure every item in a `Vec` is unique.
pub fn uniq<T>(items: Vec<T>) -> Vec<T>
where
    T: Eq + Hash + ToOwned,
    <T as ToOwned>::Owned: Hash + Eq,
{
    let mut set = HashSet::new();

    items
        .into_iter()
        .filter(|item| set.insert(item.to_owned()))
        .collect::<Vec<T>>()
}
