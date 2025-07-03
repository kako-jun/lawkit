use std::fmt;

#[derive(Debug, Clone)]
pub enum BenfError {
    InvalidInput(String),
    NetworkError(String),
    FileError(String),
    ParseError(String),
    NoNumbersFound,
    InsufficientData(usize),
}

impl fmt::Display for BenfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BenfError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            BenfError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            BenfError::FileError(msg) => write!(f, "File error: {}", msg),
            BenfError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            BenfError::NoNumbersFound => write!(f, "No numbers found in input"),
            BenfError::InsufficientData(count) => {
                write!(f, "Insufficient data for analysis: {} numbers (minimum 30 recommended)", count)
            }
        }
    }
}

impl std::error::Error for BenfError {}

pub type Result<T> = std::result::Result<T, BenfError>;

// Lawkit用のエラー型（BenfErrorのエイリアス）
pub type LawkitError = BenfError;