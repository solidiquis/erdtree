use super::File;
use crate::user::{args::{Sort, DirOrder}, Context};
use std::cmp::Ordering;

/// Comparator type used to sort [File]s.
pub type FileComparator = dyn Fn(&File, &File) -> Ordering;

/// Yields function pointer to the appropriate `File` comparator.
pub fn comparator(sort: Sort, dir_order: DirOrder) -> Option<Box<FileComparator>> {
    if matches!(sort, Sort::None) {
        return None;
    }

    match dir_order {
        DirOrder::First => {
            Some(Box::new(move |a, b| dir_first_comparator(a, b, base_comparator(sort))))
        },
        DirOrder::Last => {
            Some(Box::new(move |a, b| dir_last_comparator(a, b, base_comparator(sort))))
        },
        DirOrder::None => Some(base_comparator(sort)),
    }
}

/// Orders directories first. Provides a fallback if inputs are not directories.
fn dir_first_comparator(
    a: &File,
    b: &File,
    fallback: impl Fn(&File, &File) -> Ordering,
) -> Ordering {
    match (a.is_dir(), b.is_dir()) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        _ => fallback(a, b),
    }
}

/// Orders directories last. Provides a fallback if inputs are not directories.
fn dir_last_comparator(
    a: &File,
    b: &File,
    fallback: impl Fn(&File, &File) -> Ordering,
) -> Ordering {
    match (a.is_dir(), b.is_dir()) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => fallback(a, b),
    }
}

/// Grabs the comparator for two non-dir type [File]s.
fn base_comparator(sort_type: Sort) -> Box<FileComparator> {
    Box::new(match sort_type {
        Sort::Name => naming::comparator,
        Sort::Rname => naming::rev_comparator,
        Sort::Size => sizing::comparator,
        Sort::Rsize => sizing::rev_comparator,
        Sort::Access => time_stamping::accessed::comparator,
        Sort::Raccess => time_stamping::accessed::rev_comparator,
        Sort::Create => time_stamping::created::comparator,
        Sort::Rcreate => time_stamping::created::rev_comparator,
        Sort::Mod => time_stamping::modified::comparator,
        Sort::Rmod => time_stamping::modified::rev_comparator,

        // Hacky...
        Sort::None => unreachable!(),
    })
}

mod time_stamping {
    pub(self) use super::File;
    pub(self) use core::cmp::Ordering;
    pub(self) use std::time::SystemTime;

    pub mod accessed {
        use super::*;

        /// Comparator that sorts [File]s by Last Access timestamp, newer to older.
        pub fn comparator(a: &File, b: &File) -> Ordering {
            let a_stamp = a.metadata().accessed().unwrap_or_else(|_| SystemTime::now());
            let b_stamp = b.metadata().accessed().unwrap_or_else(|_| SystemTime::now());
            b_stamp.cmp(&a_stamp)
        }

        /// Comparator that sorts [File]s by Access timestamp, older to newer.
        pub fn rev_comparator(a: &File, b: &File) -> Ordering {
            comparator(b, a)
        }
    }

    pub mod created {
        use super::*;

        /// Comparator that sorts [File]s by Creation timestamp, newer to older.
        pub fn comparator(a: &File, b: &File) -> Ordering {
            let a_stamp = a.metadata().created().unwrap_or_else(|_| SystemTime::now());
            let b_stamp = b.metadata().created().unwrap_or_else(|_| SystemTime::now());
            b_stamp.cmp(&a_stamp)
        }

        /// Comparator that sorts [File]s by Creation timestamp, older to newer.
        pub fn rev_comparator(a: &File, b: &File) -> Ordering {
            comparator(b, a)
        }
    }

    pub mod modified {
        use super::*;

        /// Comparator that sorts [File]s by Alteration timestamp, newer to older.
        pub fn comparator(a: &File, b: &File) -> Ordering {
            let a_stamp = a.metadata().modified().unwrap_or_else(|_| SystemTime::now());
            let b_stamp = b.metadata().modified().unwrap_or_else(|_| SystemTime::now());
            b_stamp.cmp(&a_stamp)
        }

        /// Comparator that sorts [File]s by Alteration timestamp, older to newer.
        pub fn rev_comparator(a: &File, b: &File) -> Ordering {
            comparator(b, a)
        }
    }
}

mod sizing {
    use super::File;
    use std::cmp::Ordering;

    /// Comparator that sorts [File]s by size, largest to smallest.
    pub fn comparator(a: &File, b: &File) -> Ordering {
        let a_size = a.size().value();
        let b_size = b.size().value();
        b_size.cmp(&a_size)
    }
    /// Comparator that sorts [File]s by size, smallest to largest.
    pub fn rev_comparator(a: &File, b: &File) -> Ordering {
        comparator(b, a)
    }
}

mod naming {
    use super::File;
    use std::cmp::Ordering;

    /// Comparator based on [File] file names in lexicographical order.
    pub fn comparator(a: &File, b: &File) -> Ordering {
        a.file_name().cmp(b.file_name())
    }

    /// Comparator based on [File] file names in reversed lexicographical order.
    pub fn rev_comparator(a: &File, b: &File) -> Ordering {
        comparator(b, a)
    }
}
