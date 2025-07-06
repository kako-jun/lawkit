use serde_json;
use std::fmt;
use std::io; // serde_json をインポート

#[derive(Debug, Clone)]
pub enum BenfError {
    InvalidInput(String),
    NetworkError(String),
    FileError(String),
    ParseError(String),
    NoNumbersFound,
    InsufficientData(usize),
    IoError(String),            // 新しいバリアントを追加
    SerializationError(String), // 新しいバリアントを追加
}

impl fmt::Display for BenfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BenfError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            BenfError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            BenfError::FileError(msg) => write!(f, "File error: {msg}"),
            BenfError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            BenfError::NoNumbersFound => write!(f, "No numbers found in input"),
            BenfError::InsufficientData(count) => {
                write!(
                    f,
                    "Insufficient data for analysis: {count} numbers (minimum 30 recommended)"
                )
            }
            BenfError::IoError(msg) => write!(f, "I/O error: {msg}"),
            BenfError::SerializationError(msg) => write!(f, "Serialization error: {msg}"),
        }
    }
}

impl std::error::Error for BenfError {}

pub type Result<T> = std::result::Result<T, BenfError>;

// Lawkit用のエラー型（BenfErrorのエイリアス）
pub type LawkitError = BenfError;

impl From<io::Error> for BenfError {
    fn from(err: io::Error) -> Self {
        BenfError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for BenfError {
    fn from(err: serde_json::Error) -> Self {
        BenfError::SerializationError(err.to_string())
    }
}
