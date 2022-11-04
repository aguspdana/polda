use polars::prelude::CsvReader;
use polars::prelude::Field;
use polars::prelude::SerReader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::data_type::DataType;
use crate::doc::Aggregate;
use crate::doc::AggregateComputation;
use crate::doc::JoinColumn;
use crate::doc::Node;
use crate::doc::SelectColumn;
use crate::error::PoldaError;

#[derive(Debug, Clone)]
pub struct Schema(pub Arc<HashMap<String, DataType>>);

impl Schema {
    /// Validate node and return `Schema`.
    pub fn try_from_node(node: &Node, inputs: Vec<Schema>) -> Result<Schema, PoldaError> {
        match node {
            Node::Aggregate {
                id,
                position: _,
                input: _,
                aggregates,
                outputs: _
            } => {
                if inputs.len() == 0 {
                    return Err(PoldaError::QueryError(format!("AggregateNode \"{}\" is missing an input node", id)));
                }

                let schema = inputs[0].clone().0;
                let mut new_schema = HashMap::new();

                for agg in aggregates.iter() {
                    let Aggregate { column, computation, alias } = agg;
                    let dtype = schema
                        .get(column)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist in the input table for AggregateNode \"{}\"", column, id)))?;
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
                        return Err(PoldaError::QueryError(format!("Found duplicate columns \"{}\" in AggregatetNode \"{}\"", new_column, id)));
                    }
                    new_schema.insert(new_column.clone(), dtype);
                }

                Ok(Schema(Arc::new(new_schema)))
            }

            Node::Filter {
                id,
                position: _,
                input: _,
                filters,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("FilterNode \"{}\" is missing an input table", id)));
                }

                let schema = inputs[0].clone().0;

                for filter in filters.iter() {
                    let column = &filter.column;
                    if !schema.contains_key(column) {
                        return Err(PoldaError::QueryError(format!("Column \"{}\" doesn't exist in the input table for FilterNode \"{}\"", column, id)));
                    }
                }

                Ok(Schema(schema))
            }

            Node::Join {
                id,
                position: _,
                left_input: _,
                right_input: _,
                join_type: _,
                columns,
                outputs: _
            } => {
                if inputs.len() == 0 {
                    return Err(PoldaError::QueryError(format!("JoinNode \"{}\" is missing input tables", id)));
                }
                if inputs.len() == 1 {
                    return Err(PoldaError::QueryError(format!("JoinNode \"{}\" is missing an input table", id)));
                }

                let mut inputs = inputs.into_iter();
                let left_schema = inputs.next().unwrap().0;
                let right_schema = inputs.next().unwrap().0;
                let mut right_join_columns = HashSet::new();

                for join_column in columns.iter() {
                    let JoinColumn { left, right } = join_column;
                    let left_dtype = left_schema.get(left)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist in the left input table for JoinNode \"{}\"", left, id)))?;
                    let right_dtype = right_schema.get(right)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist in the right input table for JoinNode \"{}\"", right, id)))?;
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
                path,
                outputs: _
            } => {
                let frame = CsvReader::from_path(path)?
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
                id,
                position: _,
                input: _,
                columns,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("SelectNode \"{}\" is missing an input table", id)));
                }

                let schema = inputs[0].clone().0;
                let mut new_schema = HashMap::new();

                for column in columns.iter() {
                    let SelectColumn { column, alias } = column;
                    let dtype = schema.get(column)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" doesn't exist in the input table for SelectNode \"{}\"", column, id)))?;
                    let new_column = if !alias.is_empty() {
                        alias
                    } else {
                        column
                    };
                    if new_schema.contains_key(new_column) {
                        return Err(PoldaError::QueryError(format!("Found duplicate columns \"{}\" in SelectNode \"{}\"", new_column, id)));
                    }
                    new_schema.insert(new_column.clone(), dtype.clone());
                }

                Ok(Schema(Arc::new(new_schema)))
            }

            Node::Sort {
                id,
                position: _,
                input: _,
                sorters,
                outputs: _
            } => {
                if inputs.len() < 1 {
                    return Err(PoldaError::QueryError(format!("SortNode \"{}\" is missing an input table", id)));
                }

                let schema = inputs[0].clone();

                for sorter in sorters.iter() {
                    let column = &sorter.column;
                    if !schema.0.contains_key(column) {
                        return Err(PoldaError::QueryError(format!("Column \"{}\" doesn't exist in the input table for SortNode \"{}\"", column, id)));
                    }
                }
                Ok(schema)
            }

            Node::Union {
                id,
                position: _,
                primary_input: _,
                secondary_input: _,
                outputs: _
            } => {
                if inputs.len() == 0 {
                    return Err(PoldaError::QueryError(format!("UnionNode \"{}\" is missing input tables", id)));
                }
                if inputs.len() == 1 {
                    return Err(PoldaError::QueryError(format!("UnionNode \"{}\" is missing an input table", id)));
                }

                let mut inputs = inputs.into_iter();
                let primary_schema = inputs.next().unwrap().0;
                let secondary_schema = inputs.next().unwrap().0;

                for (sec_col, sec_type) in secondary_schema.iter() {
                    let pri_type = primary_schema.get(sec_col)
                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" is missing in the first input table for UnionNode \"{}\"", sec_col, id)))?;
                    if pri_type != sec_type {
                        return Err(PoldaError::QueryError(format!("Column \"{}\" has different types in the input tables for UnionNode \"{}\"", sec_col, id)));
                    }
                }

                for (pri_col, _) in primary_schema.iter() {
                    if !secondary_schema.contains_key(pri_col) {
                        return Err(PoldaError::QueryError(format!("Column \"{}\" is missing in the second input table for UnionNode \"{}\"", pri_col, id)));
                    }
                }

                Ok(Schema(primary_schema))
            }
        }
    }
}
