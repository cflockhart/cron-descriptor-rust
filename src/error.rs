use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum CronError {
    #[error("Invalid expression: {expression}")]
    InvalidExpression { expression: String },

    #[error("Parse error at position {error_offset}: {message}")]
    ParseException { message: String, error_offset: u8 },

    #[error("Formatting error: {0}")]
    FormattingError(String),
}

pub type Result<T> = std::result::Result<T, CronError>;
