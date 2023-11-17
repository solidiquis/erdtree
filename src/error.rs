use ansi_term::{Color, Style};
use std::{convert::From, error::Error as StdError, fmt, result::Result as StdResult};

/// Meant to be a convenient wild-card import to allow access to the [`crate::error`] module's facilities
/// error-handling facilities.
pub mod prelude {
    pub use super::{Error, ErrorCategory, ErrorReport, Result, WithContext};

    macro_rules! error_source {
        () => {
            format!("{}:{}", file!(), line!())
        };
    }
    pub(crate) use error_source;
}

/// General result type to be used through the application.
pub type Result<T> = std::result::Result<T, Error>;

/// General error type to be used throughout the application. Depending on the `category`, a
/// different format of the error will be presented to the end-user.
#[derive(Debug)]
pub struct Error {
    source: anyhow::Error,
    category: ErrorCategory,
    help_text: Option<String>,
}

/// ErrorCategory of errors with which to generate a report.
#[derive(Debug)]
pub enum ErrorCategory {
    /// Errors due to logical errors within the application. When creating an [`Error`] via
    /// [`ErrorReport`], an `Internal` error will come with a default help text to guide the user
    /// to the Github issues page to file a new issue. Becareful when overriding help texts under
    /// these circumstances.
    Internal,

    /// User-specific errors to be used in relation to command-line arguments and configs.
    User,

    /// Errors related to environment such as the missing of the `$HOME` environment variable.
    System,

    /// Errors that are meant to be recoverable.
    Warning,
}

impl Error {
    pub fn new(category: ErrorCategory, source: anyhow::Error, help_text: Option<String>) -> Self {
        Self {
            source,
            category,
            help_text,
        }
    }

    fn internal_error_help_message() -> String {
        format!(
            "Please submit the error output to {}",
            Style::default()
                .bold()
                .paint("https://github.com/solidiquis/erdtree/issues")
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = Color::Red.bold().paint("\u{2715}");
        let prefix = Style::default().bold().paint(format!("{}", self.category));
        let help = Color::Cyan.bold().paint("help");

        if let Some(ref help_txt) = self.help_text {
            writeln!(
                f,
                "{icon} {prefix}: {:?}\n\n{help}: {help_txt}",
                self.source
            )
        } else {
            writeln!(f, "{} {:?}", icon, self.source)
        }
    }
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal => write!(f, "Internal error"),
            Self::User => write!(f, "Error"),
            Self::System => write!(f, "System error"),
            Self::Warning => write!(f, ""),
        }
    }
}

/// Convenience trait to generate a [`Result`] from any type that implements [`std::error::Error`].
pub trait ErrorReport<T> {
    fn into_report(self, category: ErrorCategory) -> Result<T>;
}

/// Allows the chaining of contexts to [`Error`]'s underlying [`anyhow::Error`].
pub trait WithContext<T, C>
where
    C: fmt::Display + Send + Sync + 'static,
{
    /// Chain together contexts.
    fn context(self, ctx: C) -> Result<T>;

    /// Set some help text to display with the error output.
    fn set_help(self, msg: C) -> Result<T>;
}

impl<T, E: StdError + Send + Sync + 'static> ErrorReport<T> for StdResult<T, E> {
    fn into_report(self, category: ErrorCategory) -> Result<T> {
        self.map_err(|e| {
            let help_text = matches!(category, ErrorCategory::Internal)
                .then(Error::internal_error_help_message);

            let anyhow_err = anyhow::Error::from(e);
            Error::new(category, anyhow_err, help_text)
        })
    }
}

impl<T, C> WithContext<T, C> for Result<T>
where
    C: fmt::Display + Send + Sync + 'static,
{
    fn context(self, ctx: C) -> Self {
        self.map_err(|mut err| {
            let cause = err.source.context(ctx);
            err.source = cause;
            err
        })
    }

    fn set_help(self, msg: C) -> Self {
        self.map_err(|mut err| {
            err.help_text = Some(format!("{msg}"));
            err
        })
    }
}
