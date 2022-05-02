use std::fmt;
use std::ops::Sub;

#[derive(Clone)]
pub enum UnitPrefix {
    None, // vanilla bytes
    Kilo,
    Mega,
    Giga
}

impl fmt::Debug for UnitPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitPrefix::None => f.write_str("B"),
            UnitPrefix::Kilo => f.write_str("KB"),
            UnitPrefix::Mega => f.write_str("MB"),
            UnitPrefix::Giga => f.write_str("GB"),
        }
    }
} 

impl Sub for UnitPrefix {
    type Output = i8;

    /// How many dimensional analysis steps does it take to convert
    /// rhs to ls where:
    ///     lhs: from
    ///     rhs: to
    fn sub(self, other: Self) -> Self::Output {
        match self {
            UnitPrefix::Giga => match other {
                UnitPrefix::Giga => 0,
                UnitPrefix::Mega => -1,
                UnitPrefix::Kilo => -2,
                UnitPrefix::None => -3
            },

            UnitPrefix::Mega => match other {
                UnitPrefix::Giga => 1,
                UnitPrefix::Mega => 0,
                UnitPrefix::Kilo => -1,
                UnitPrefix::None => -2
            },

            UnitPrefix::Kilo => match other {
                UnitPrefix::Giga => 2,
                UnitPrefix::Mega => 1,
                UnitPrefix::Kilo => 0,
                UnitPrefix::None => -1
            },

            UnitPrefix::None => match other {
                UnitPrefix::Giga => 3,
                UnitPrefix::Mega => 2,
                UnitPrefix::Kilo => 1,
                UnitPrefix::None => 0
            }
        }
    }
}

/// Arbitrarily determines which byte unit is most presentable from vanilla bytes.
pub fn pretty_unit(bytes: u64) -> UnitPrefix {
    if bytes > 1024 * 1_000_000 {
        UnitPrefix::Giga
    } else if bytes > 1024 * 1000 {
        UnitPrefix::Mega
    } else if bytes > 1024 {
        UnitPrefix::Kilo
    } else {
        UnitPrefix::None
    }
}

/// Utility to convert between byte units.
pub fn convert(xbytes: u64, from: UnitPrefix, to: UnitPrefix) -> f64 {
    let mut current_unit = from.clone();
    let mut num_steps = from - to;
    
    let mut result = xbytes as f64;

    if num_steps > 0 {

        while num_steps > 0 {

            if let UnitPrefix::None = current_unit {
                result /= 1024_f64;
                current_unit = UnitPrefix::Kilo;
            } else {
                result /= 1000_f64
            }

            num_steps -= 1;
        }

    } else {

        while num_steps < 0 {

            if let UnitPrefix::Kilo = current_unit {
                result *= 1024_f64;
                current_unit = UnitPrefix::None;
            } else {
                result *= 1000_f64
            }

            num_steps += 1;
        }

    }

    result
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_steps() {
        use super::UnitPrefix;

        assert_eq!(UnitPrefix::None - UnitPrefix::None, 0);
        assert_eq!(UnitPrefix::Giga - UnitPrefix::Giga, 0);
        assert_eq!(UnitPrefix::Giga - UnitPrefix::Mega, -1);
        assert_eq!(UnitPrefix::Mega - UnitPrefix::Giga, 1);
    }

    #[test]
    fn test_convert() {
        use super::UnitPrefix;
        use super::convert;

        assert_eq!(
            convert(1024, UnitPrefix::None, UnitPrefix::Kilo),
            1_f64
        );

        assert_eq!(
            convert(2, UnitPrefix::Giga, UnitPrefix::Mega),
            2000_f64 
        );

        assert_eq!(
            convert(10240000, UnitPrefix::None, UnitPrefix::Mega),
            10_f64 
        );
    }
}
