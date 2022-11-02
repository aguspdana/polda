use polars::prelude::PolarsError;

#[derive(Debug)]
pub enum PoldaError {
    DocError(String),
    ParseError(String),
    PolarsError(PolarsError),
    QueryError(String),
    OperationError(String),
}

impl From<PolarsError> for PoldaError {
    fn from(error: PolarsError) -> PoldaError {
        PoldaError::PolarsError(error)
    }
}
