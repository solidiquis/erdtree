use lscolors::Color;
use std::fmt::{self, Display, Formatter};

/// Responsible for displaying bytes in human-readable format using the largest appropriate SI
/// prefix.
#[derive(Debug)]
pub struct FileSize {
    bytes: u64
}

/// SI prefixes.
#[derive(Debug)]
pub enum Prefix {
    Base,
    Kilo,
    Mega,
    Giga
}

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Base => write!(f, "B"),
            Prefix::Kilo => write!(f, "KB"),
            Prefix::Mega => write!(f, "MB"),
            Prefix::Giga => write!(f, "GB"),
        }
    }
}

impl FileSize {
    /// Initializes [FileSize] from file-size in bytes.
    pub fn new(bytes: u64) -> Self {
        Self { bytes }
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let fbytes = self.bytes as f64;
        let log = fbytes.log(10.0);

        let output = if log < 3.0 {
            Color::BrightCyan.to_ansi_term_color().paint(format!("{:.2} {}", fbytes, Prefix::Base))
        } else if log >= 3.0 && log < 6.0 {
            Color::BrightYellow.to_ansi_term_color().paint(format!("{:.2} {}", fbytes / 1_000.0, Prefix::Kilo))
        } else if log >= 6.0 && log < 9.0 {
            Color::BrightGreen.to_ansi_term_color().paint(format!("{:.2} {}", fbytes / 1_000_000.0, Prefix::Mega))
        } else {
            Color::BrightRed.to_ansi_term_color().paint(format!("{:.2} {}", fbytes / 1_000_000_000.0, Prefix::Giga))
        };

        write!(f, "{output}")
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_file_size_display() {
        use lscolors::Color;
        use super::FileSize;

        let b = FileSize::new(100);
        assert_eq!(
            format!("{}", b),
            format!("{}", Color::BrightCyan.to_ansi_term_color().paint("100.00 B"))
        );

        let kb = FileSize::new(1_100);
        assert_eq!(
            format!("{}", kb),
            format!("{}", Color::BrightYellow.to_ansi_term_color().paint("1.10 KB"))
        );

        let mb = FileSize::new(1_100_000);
        assert_eq!(
            format!("{}", mb),
            format!("{}", Color::BrightGreen.to_ansi_term_color().paint("1.10 MB"))
        );

        let gb = FileSize::new(1_120_000_000);
        assert_eq!(
            format!("{}", gb),
            format!("{}", Color::BrightRed.to_ansi_term_color().paint("1.12 GB"))
        );
    }
}
