#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[error("Expected #{0} to be initialized")]
    Uninitialized(&'a str),
}
