use polars::prelude::PolarsError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PoldaError {
    DocError(String),
    ParseError(String),
    PolarsError(PolarsError),
    QueryError(String),
    OperationError(String),
    InternalError(String)
}

impl fmt::Display for PoldaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PoldaError::*;
        match self {
            DocError(msg) => write!(f, "DocError: {}", msg),
            InternalError(msg) => write!(f, "InternalError: {}", msg),
            ParseError(msg) => write!(f, "ParseError: {}", msg),
            PolarsError(e) => write!(f, "PolarsError: {}", e),
            QueryError(msg) => write!(f, "QueryError: {}", msg),
            OperationError(msg) => write!(f, "OperationError: {}", msg)
        }
    }
}

impl Error for PoldaError {}

impl From<PolarsError> for PoldaError {
    fn from(error: PolarsError) -> PoldaError {
        PoldaError::PolarsError(error)
    }
}
