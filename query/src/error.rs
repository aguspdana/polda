use polars::prelude::PolarsError;

#[derive(Debug)]
pub enum PoldaError {
    PolarsError(PolarsError),
    QueryError,
    Unsyncable,
}

impl From<PolarsError> for PoldaError {
    fn from(error: PolarsError) -> PoldaError {
        PoldaError::PolarsError(error)
    }
}
