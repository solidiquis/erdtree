use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct FileSize {
    bytes: usize,
    presentation: Presentation,
}

#[derive(Debug)]
pub enum Presentation {
    Base,
    Human,
}

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
    pub fn new(bytes: usize, presentation: Presentation) -> Self {
        Self { bytes, presentation }
    }

    pub fn new_base(bytes: usize) -> Self {
        Self { bytes, presentation: Presentation::Base }
    }

    pub fn new_human_readable(bytes: usize) -> Self {
        Self { bytes, presentation: Presentation::Human }
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let fbytes = self.bytes as f64;
        let log = fbytes.log(10.0);

        if log < 3.0 {
            write!(f, "{:.2}{}", fbytes, Prefix::Base)
        } else if log >= 3.0 && log < 6.0 {
            write!(f, "{:.2}{}", fbytes / 1_000.0, Prefix::Kilo)
        } else if log >= 6.0 && log < 9.0 {
            write!(f, "{:.2}{}", fbytes / 1_000_000.0, Prefix::Mega)
        } else {
            write!(f, "{:.2}{}", fbytes / 1_000_000_000.0, Prefix::Giga)
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_file_size_display() {
        use super::FileSize;

        let b = FileSize::new_human_readable(100);
        assert_eq!(format!("{}", b), "100.00B".to_owned());

        let kb = FileSize::new_human_readable(1_100);
        assert_eq!(format!("{}", kb), "1.10KB".to_owned());

        let mb = FileSize::new_human_readable(1_100_000);
        assert_eq!(format!("{}", mb), "1.10MB".to_owned());

        let gb = FileSize::new_human_readable(1_120_000_000);
        assert_eq!(format!("{}", gb), "1.12GB".to_owned());
    }
}
