use polars::lazy::prelude::lit;
use polars::prelude::CsvReader;
use polars::prelude::Expr;
use polars::prelude::Field;
use polars::prelude::SerReader;
use std::collections::HashMap;

use crate::data_type::DataType;
use crate::error::PoldaError;

pub fn get_csv_schema(path: &str) -> Result<HashMap<String, DataType>, PoldaError> {
    let frame = CsvReader::from_path(path)?
        .with_n_rows(Some(100))
        .finish()?;
    let mut schema = HashMap::new();
    for field in frame.schema().iter_fields() {
        let Field { name, dtype } = field;
        let dtype = DataType::try_from(dtype)?;
        schema.insert(name, dtype);
    }
    Ok(schema)
}

pub fn parse_constant_expr(
    constant: &str,
    dtype: &DataType
) -> Result<Expr, PoldaError> {
    match dtype {
        DataType::Boolean => {
            constant.parse::<bool>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Boolean", constant)))
        }
        DataType::Float32 => {
            constant.parse::<f32>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Float32", constant)))
        }
        DataType::Float64 => {
            constant.parse::<f64>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Float64", constant)))
        }
        DataType::Int8 => {
            constant.parse::<i8>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Int8", constant)))
        }
        DataType::Int16 => {
            constant.parse::<i16>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Int16", constant)))
        }
        DataType::Int32 => {
            constant.parse::<i32>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Int32", constant)))
        }
        DataType::Int64 => {
            constant.parse::<i64>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Int64", constant)))
        }
        DataType::UInt8 => {
            constant.parse::<u8>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a UInt8", constant)))
        }
        DataType::UInt16 => {
            constant.parse::<u16>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a UInt16", constant)))
        }
        DataType::UInt32 => {
            constant.parse::<u32>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a UInt32", constant)))
        }
        DataType::UInt64 => {
            constant.parse::<u64>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a UInt64", constant)))
        }
        DataType::Utf8 => {
            Ok(lit(constant))
        }
        _ => todo!()
    }
}
