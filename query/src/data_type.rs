use polars::datatypes::DataType as PolarsDataType;
use serde::Deserialize;
use serde::Serialize;

use crate::error::PoldaError;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Boolean,
    Date,
    DateTime,
    Duration,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    List(Box<DataType>),
    Time,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Utf8
}

impl DataType {
    pub fn into_polars(&self) -> PolarsDataType {
        match self {
            DataType::Boolean => PolarsDataType::Boolean,
            DataType::Date => PolarsDataType::Date,
            DataType::Float32 => PolarsDataType::Float32,
            DataType::Float64 => PolarsDataType::Float64,
            DataType::Int8 => PolarsDataType::Int8,
            DataType::Int16 => PolarsDataType::Int16,
            DataType::Int32 => PolarsDataType::Int32,
            DataType::Int64 => PolarsDataType::Int64,
            DataType::UInt8 => PolarsDataType::UInt8,
            DataType::UInt16 => PolarsDataType::UInt16,
            DataType::UInt32 => PolarsDataType::UInt32,
            DataType::UInt64 => PolarsDataType::UInt64,
            DataType::Utf8 => PolarsDataType::Utf8,
            _ => todo!()
        }
    }
}

impl TryFrom<PolarsDataType> for DataType {
    type Error = PoldaError;

    fn try_from(dtype: PolarsDataType) -> Result<DataType, PoldaError> {
        match dtype {
            PolarsDataType::Boolean => Ok(DataType::Boolean),
            PolarsDataType::Date => Ok(DataType::Date),
            PolarsDataType::Float32 => Ok(DataType::Float32),
            PolarsDataType::Float64 => Ok(DataType::Float64),
            PolarsDataType::Int8 => Ok(DataType::Int8),
            PolarsDataType::Int16 => Ok(DataType::Int16),
            PolarsDataType::Int32 => Ok(DataType::Int32),
            PolarsDataType::Int64 => Ok(DataType::Int64),
            PolarsDataType::UInt8 => Ok(DataType::UInt8),
            PolarsDataType::UInt16 => Ok(DataType::UInt16),
            PolarsDataType::UInt32 => Ok(DataType::UInt32),
            PolarsDataType::UInt64 => Ok(DataType::UInt64),
            PolarsDataType::Utf8 => Ok(DataType::Utf8),
            _ => todo!()
        }
    }
}
