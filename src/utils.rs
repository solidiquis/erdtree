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

/// Follow the naming convention and use "-" to specify a Standard Input.
/// Retain "-" from [`args`] and add "--stdin" if necessary.
pub fn detect_stdin(args: &mut Vec<String>) {
    let dash = String::from("-");
    let stdin_flag = String::from("--stdin");

    let mut is_stdin = false;
    args.retain(|e| {
        if *e == dash {
            is_stdin = true
        };
        *e != dash
    });
    if is_stdin && !args.contains(&stdin_flag) {
        args.push(stdin_flag)
    }
}
