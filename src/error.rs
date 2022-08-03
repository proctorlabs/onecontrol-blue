pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Display)]
pub enum AppError {
    InvalidPayload,
    InvalidCommand(u8),
    Generic(String),
}

impl<T: Into<anyhow::Error>> From<T> for AppError {
    fn from(e: T) -> Self {
        let e: anyhow::Error = e.into();
        Self::Generic(e.to_string())
    }
}
