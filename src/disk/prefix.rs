use std::{
    convert::From,
    fmt::{self, Display},
};

/// <https://en.wikipedia.org/wiki/Binary_prefix>
#[derive(Debug, PartialEq)]
pub enum Binary {
    Base,
    Kibi,
    Mebi,
    Gibi,
    Tebi,
    Pebi,
}

/// <https://en.wikipedia.org/wiki/International_System_of_Units>
#[derive(Debug, PartialEq)]
pub enum Si {
    Base,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
}

impl Binary {
    pub fn base_value(&self) -> f64 {
        match self {
            Self::Base => 1.,
            Self::Kibi => 2_u64.pow(10) as f64,
            Self::Mebi => 2_u64.pow(20) as f64,
            Self::Gibi => 2_u64.pow(30) as f64,
            Self::Tebi => 2_u64.pow(40) as f64,
            Self::Pebi => 2_u64.pow(50) as f64,
        }
    }
}

impl Si {
    pub fn base_value(&self) -> f64 {
        match self {
            Self::Base => 1.,
            Self::Kilo => 10_u64.pow(3) as f64,
            Self::Mega => 10_u64.pow(6) as f64,
            Self::Giga => 10_u64.pow(9) as f64,
            Self::Tera => 10_u64.pow(12) as f64,
            Self::Peta => 10_u64.pow(15) as f64,
        }
    }
}

impl From<u64> for Binary {
    fn from(bytes: u64) -> Self {
        let bytes = (bytes as f64).log2();

        if bytes < 10. {
            Self::Base
        } else if bytes < 20. {
            Self::Kibi
        } else if bytes < 30. {
            Self::Mebi
        } else if bytes < 40. {
            Self::Gibi
        } else if bytes < 50. {
            Self::Tebi
        } else {
            Self::Pebi
        }
    }
}

impl From<u64> for Si {
    fn from(bytes: u64) -> Self {
        let bytes = (bytes as f64).log10();

        if bytes < 3. {
            Self::Base
        } else if bytes < 6. {
            Self::Kilo
        } else if bytes < 9. {
            Self::Mega
        } else if bytes < 12. {
            Self::Giga
        } else if bytes < 15. {
            Self::Tera
        } else {
            Self::Peta
        }
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, ""),
            Self::Kibi => write!(f, "Ki"),
            Self::Mebi => write!(f, "Mi"),
            Self::Gibi => write!(f, "Gi"),
            Self::Tebi => write!(f, "Ti"),
            Self::Pebi => write!(f, "Pi"),
        }
    }
}

impl Display for Si {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, ""),
            Self::Kilo => write!(f, "K"),
            Self::Mega => write!(f, "M"),
            Self::Giga => write!(f, "G"),
            Self::Tera => write!(f, "T"),
            Self::Peta => write!(f, "P"),
        }
    }
}

#[test]
fn test_prefixes() {
    assert_eq!(Binary::from(100), Binary::Base);
    assert_eq!(Binary::from(2_u64.pow(10)), Binary::Kibi);
    assert_eq!(Binary::from(2_u64.pow(20)), Binary::Mebi);
    assert_eq!(Binary::from(2_u64.pow(30)), Binary::Gibi);
    assert_eq!(Binary::from(2_u64.pow(40)), Binary::Tebi);
    assert_eq!(Binary::from(2_u64.pow(50)), Binary::Pebi);

    assert_eq!(Si::from(100), Si::Base);
    assert_eq!(Si::from(10_u64.pow(3)), Si::Kilo);
    assert_eq!(Si::from(10_u64.pow(6)), Si::Mega);
    assert_eq!(Si::from(10_u64.pow(9)), Si::Giga);
    assert_eq!(Si::from(10_u64.pow(12)), Si::Tera);
    assert_eq!(Si::from(10_u64.pow(15)), Si::Peta);
}
