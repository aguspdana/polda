use polars::prelude::CsvReader;
use polars::prelude::Field;
use polars::prelude::SerReader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::data_type::DataType;
use crate::doc::Aggregate;
use crate::doc::AggregateComputation;
use crate::doc::ComputeOperation;
use crate::doc::JoinColumn;
use crate::doc::Node;
use crate::doc::SelectColumn;
use crate::doc::Value;
use crate::error::PoldaError;

#[derive(Debug, Clone)]
pub struct Schema(pub Arc<HashMap<String, DataType>>);

impl Schema {
    /// Validate node and return `Schema`.
    pub fn try_from_node(node: &Node, inputs: Vec<Schema>) -> Result<Schema, PoldaError> {
        match node {
            Node::Aggregate {
                id: _,
                position: _,
                input: _,
                aggregates,
                outputs: _
            } => {
                if inputs.len() == 0 {
                    return Err(PoldaError::QueryError(format!("AggregateNode is missing an input node")));
                }

                let schema = inputs[0].clone().0;
                let mut new_schema = HashMap::new();

                for agg in aggregates.iter() {
                    let Aggregate { column, computation, alias } = agg;
                    let dtype = schema
                        .get(column)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)))?;
                    let dtype = match computation {
                        AggregateComputation::Count => {
                            DataType::UInt32
                        }
                        _ => dtype.clone(),
                    };
                    let new_column = if alias.is_empty() {
                        column
                    } else {
                        alias
                    };
                    if new_schema.contains_key(new_column) {
                        return Err(PoldaError::QueryError(format!("Found duplicate columns \"{}\"", new_column)));
                    }
                    new_schema.insert(new_column.clone(), dtype);
                }

                Ok(Schema(Arc::new(new_schema)))
            }

            Node::Bins {
                id: _,
                position: _,
                input: _,
                name,
                column,
                lower_bound: _,
                upper_bound: _,
                count: _,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("BinsNode is missing an input table")));
                }

                let mut schema = inputs[0].clone().0.as_ref().clone();

                let dtype = schema
                    .get(column)
                    .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)))?
                    .clone();
                schema.insert(name.clone(), dtype);

                Ok(Schema(Arc::new(schema)))
            }

            Node::Case {
                id: _,
                position: _,
                input: _,
                name,
                data_type,
                cases,
                default: _,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("CaseNode is missing an input table")));
                }

                let mut schema = inputs[0].clone().0.as_ref().clone();

                // Validate cases.
                for case in cases.iter() {
                    let dtype = schema
                        .get(&case.column)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", case.column)))?;
                    if dtype != &DataType::Boolean {
                        return Err(PoldaError::QueryError(format!("Case column \"{}\" is not a boolean", case.column)));
                    }
                    if let Value::Column(col) = &case.value {
                        if let Some(dt) = schema.get(&**col) {
                            if dt != data_type {
                                return Err(PoldaError::QueryError(format!("Case value \"{}\" has incompatible data type", col)));
                            }
                        } else {
                            return Err(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", col)));
                        }
                    }
                }

                schema.insert(name.clone(), data_type.clone());

                Ok(Schema(Arc::new(schema)))
            }

            Node::Cast {
                id: _,
                position: _,
                input: _,
                name,
                column,
                data_type,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("CastNode is missing an input table")));
                }

                let mut schema = inputs[0].clone().0.as_ref().clone();

                if !schema.contains_key(column) {
                    return Err(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)));
                }

                schema.insert(name.clone(), data_type.clone());

                Ok(Schema(Arc::new(schema)))
            }

            Node::Compute {
                id: _,
                position: _,
                input: _,
                name,
                column,
                operation,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("CastNode is missing an input table")));
                }

                let mut schema = inputs[0].clone().0.as_ref().clone();

                macro_rules! insert_if_exists {
                    ($schema:ident, $col:ident) => {{
                        let dtype = schema
                            .get(column)
                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)))?
                            .clone();
                        schema.insert(name.clone(), dtype);
                    }};
                }

                macro_rules! insert_dtype_if_exists {
                    ($schema:ident, $col:ident, $dtype:expr) => {
                        if !schema.contains_key(column) {
                            return Err(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)));
                        }
                        schema.insert(name.clone(), $dtype);
                    };
                }

                macro_rules! insert_if_same_type {
                    ($schema:ident, $col:ident, $other_col:ident) => {{
                        let dtype = schema
                            .get(column)
                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)))?
                            .clone();
                        let other_dtype = $schema
                            .get($other_col)
                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", $other_col)))?;
                        if &dtype != other_dtype {
                            return Err(PoldaError::QueryError(format!("Column \"{}\" and \"{}\" have different data types", $col, $other_col)));
                        }
                        schema.insert(name.clone(), dtype);
                    }};
                }

                macro_rules! insert_dtype_if_same_type {
                    ($schema:ident, $col:ident, $other_col:ident, $dtype:expr) => {
                        let dtype = schema
                            .get(column)
                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)))?;
                        let other_dtype = $schema
                            .get($other_col)
                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", $other_col)))?;
                        if dtype != other_dtype {
                            return Err(PoldaError::QueryError(format!("Column \"{}\" and \"{}\" have different data types", $col, $other_col)));
                        }
                        schema.insert(name.clone(), $dtype);
                    };
                }

                macro_rules! insert_bool_if_bool {
                    ($schema:ident, $col:ident) => {{
                        let dtype = schema
                            .get(column)
                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)))?;
                        if dtype != &DataType::Boolean {
                            return Err(PoldaError::QueryError(format!("Column \"{}\" is not a boolean", $col)));
                        }
                        schema.insert(name.clone(), DataType::Boolean);
                    }};

                    ($schema:ident, $col:ident, $other_col:ident) => {{
                        let dtype = schema
                            .get(column)
                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)))?;
                        if dtype != &DataType::Boolean {
                            return Err(PoldaError::QueryError(format!("Column \"{}\" is not a boolean", $col)));
                        }
                        let other_dtype = $schema
                            .get($other_col)
                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", $other_col)))?;
                        if other_dtype != &DataType::Boolean {
                            return Err(PoldaError::QueryError(format!("Column \"{}\" is not a boolean", $col)));
                        }
                        schema.insert(name.clone(), DataType::Boolean);
                    }};
                }

                use ComputeOperation::*;
                match operation {
                    Add(Value::Column(col))
                        | Subtract(Value::Column(col))
                        | Multiply(Value::Column(col))
                        | Divide(Value::Column(col)) => {
                        insert_if_same_type!(schema, column, col);
                    }
                    Add(_)
                        | Subtract(_)
                        | Multiply(_)
                        | Divide(_)
                        | Mean
                        | Median
                        | Min
                        | Max => {
                        insert_if_exists!(schema, column);
                    }
                    IsEqualTo(Value::Column(col))
                        | IsNotEqualTo(Value::Column(col))
                        | IsLessThan(Value::Column(col))
                        | IsLessThanEqual(Value::Column(col))
                        | IsGreaterThan(Value::Column(col))
                        | IsGreaterThanEqual(Value::Column(col)) => {
                        insert_dtype_if_same_type!(schema, column, col, DataType::Boolean);
                    }
                    IsEqualTo(_)
                        | IsNotEqualTo(_)
                        | IsLessThan(_)
                        | IsLessThanEqual(_)
                        | IsGreaterThan(_)
                        | IsGreaterThanEqual(_)
                        | IsNull
                        | IsNotNull => {
                        insert_dtype_if_exists!(schema, column, DataType::Boolean);
                    }
                    And(Value::Column(col))
                        | Or(Value::Column(col))
                        | Xor(Value::Column(col)) => {
                        insert_bool_if_bool!(schema, column, col);
                    }
                    And(_)
                        | Or(_)
                        | Xor(_) => {
                        insert_bool_if_bool!(schema, column);
                    }
                }

                Ok(Schema(Arc::new(schema)))
            }


            Node::Filter {
                id: _,
                position: _,
                input: _,
                column,
                predicate: _,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("FilterNode is missing an input table")));
                }

                let schema = inputs[0].clone().0;

                if !schema.contains_key(column) {
                    return Err(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)));
                }

                Ok(Schema(schema))
            }

            Node::Join {
                id: _,
                position: _,
                left_input: _,
                right_input: _,
                join_type: _,
                columns,
                outputs: _
            } => {
                if inputs.len() < 2 {
                    return Err(PoldaError::QueryError(format!("JoinNode is missing an input table")));
                }

                let mut inputs = inputs.into_iter();
                let left_schema = inputs.next().unwrap().0;
                let right_schema = inputs.next().unwrap().0;
                let mut right_join_columns = HashSet::new();

                for join_column in columns.iter() {
                    let JoinColumn { left, right } = join_column;
                    let left_dtype = left_schema.get(left)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist in the left input table", left)))?;
                    let right_dtype = right_schema.get(right)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist in the right input table", right)))?;
                    if left_dtype != right_dtype {
                        return Err(PoldaError::QueryError(format!("Join columns \"{}\" and \"{}\" have different data types", left, right)));
                    }
                    right_join_columns.insert(right.clone());
                }

                let mut new_schema = left_schema.as_ref().clone();

                for (column, dtype) in right_schema.iter() {
                    if !new_schema.contains_key(column) {
                        new_schema.insert(column.clone(), dtype.clone());
                    }
                }

                Ok(Schema(Arc::new(new_schema)))
            }

            Node::LoadCsv {
                id: _,
                position: _,
                filename,
                outputs: _
            } => {
                let frame = CsvReader::from_path(filename)?
                    .with_n_rows(Some(100))
                    .finish()?;
                let mut schema = HashMap::new();
                for field in frame.schema().iter_fields() {
                    let Field { name, dtype } = field;
                    let dtype = DataType::try_from(dtype)?;
                    schema.insert(name, dtype);
                }
                Ok(Schema(Arc::new(schema)))
            }

            Node::Select {
                id: _,
                position: _,
                input: _,
                columns,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("SelectNode is missing an input table")));
                }

                let schema = inputs[0].clone().0;
                let mut new_schema = HashMap::new();

                for column in columns.iter() {
                    let SelectColumn { column, alias } = column;
                    let dtype = schema.get(column)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)))?;
                    let new_column = if !alias.is_empty() {
                        alias
                    } else {
                        column
                    };
                    if new_schema.contains_key(new_column) {
                        return Err(PoldaError::QueryError(format!("Found duplicate columns \"{}\" in SelectNode", new_column)));
                    }
                    new_schema.insert(new_column.clone(), dtype.clone());
                }

                Ok(Schema(Arc::new(new_schema)))
            }

            Node::Sort {
                id: _,
                position: _,
                input: _,
                sorters,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("SortNode is missing an input table")));
                }

                let schema = inputs[0].clone();

                for sorter in sorters.iter() {
                    let column = &sorter.column;
                    if !schema.0.contains_key(column) {
                        return Err(PoldaError::QueryError(format!("Column \"{}\" doesn't exist", column)));
                    }
                }
                Ok(schema)
            }

            Node::Union {
                id: _,
                position: _,
                primary_input: _,
                secondary_input: _,
                outputs: _
            } => {
                if inputs.len() < 2 {
                    return Err(PoldaError::QueryError(format!("UnionNode is missing an input table")));
                }

                let mut inputs = inputs.into_iter();
                let primary_schema = inputs.next().unwrap().0;
                let secondary_schema = inputs.next().unwrap().0;

                for (sec_col, sec_type) in secondary_schema.iter() {
                    let pri_type = primary_schema.get(sec_col)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" is missing in the first input table", sec_col)))?;
                    if pri_type != sec_type {
                        return Err(PoldaError::QueryError(format!("Column \"{}\" has different types", sec_col)));
                    }
                }

                for (pri_col, _) in primary_schema.iter() {
                    if !secondary_schema.contains_key(pri_col) {
                        return Err(PoldaError::QueryError(format!("Column \"{}\" is missing in the second input table", pri_col)));
                    }
                }

                Ok(Schema(primary_schema))
            }
        }
    }
}
