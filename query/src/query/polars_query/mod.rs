use polars::frame::DataFrame;
use polars::lazy::frame::LazyFrame;
use polars::prelude::col;
use polars::prelude::LazyCsvReader;
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::PoldaError;
use crate::data_type::DataType;
use crate::doc::Node;
use crate::doc::Filter;
use crate::doc::FilterPredicate;
use crate::doc::Value;
use super::Schema;

mod utils;

use utils::get_csv_schema;
use utils::parse_constant_expr;

#[derive(Clone)]
pub struct PolarsQuery {
    pub frame: LazyFrame,
    pub schema: Arc<Schema>
}

impl PolarsQuery {
    pub fn collect(self) -> Result<DataFrame, PoldaError> {
        Ok(self.frame.collect()?)
    }

    pub fn from_node(
        node: &Node,
        mut inputs: Vec<PolarsQuery>
    ) -> Result<PolarsQuery, PoldaError> {
        match node {
            Node::Filter {
                id,
                position: _,
                input: _,
                filters,
                outputs: _
            } => {
                if let Some(input) = inputs.pop() {
                    let PolarsQuery { frame, schema } = input;
                    let mut exprs = vec![];
                    for filter in filters.iter() {
                        let Filter { column, predicate } = filter;
                        let col_expr = col(column);
                        match predicate {
                            FilterPredicate::IsEqualTo(value) => {
                                let comp_expr = match value {
                                    Value::Column(column) => col(column),
                                    Value::Constant(constant) => {
                                        schema
                                            .get(column)
                                            .map(|dtype| parse_constant_expr(constant.as_str(), dtype))
                                            .ok_or(PoldaError::QueryError(format!("Column \"{}\" does not exist", {column})))??
                                    }
                                };
                                let expr = col_expr.eq(comp_expr);
                                exprs.push(expr);
                            }

                            _ => todo!()
                        }
                    }
                    let frame = exprs
                        .into_iter()
                        .fold(frame, |frame, filter| {
                            frame.filter(filter)
                        });
                    Ok(PolarsQuery { frame, schema })
                } else {
                    Err(PoldaError::QueryError(format!("Node {} is missing an input table", id)))
                }
            }

            Node::LoadCsv {
                id: _,
                position: _,
                path,
                outputs: _
            } => {
                let frame = LazyCsvReader::new(path).finish()?;
                let columns = get_csv_schema(&*path)?;
                let query = PolarsQuery::new(frame, Arc::new(columns));
                Ok(query)
            }

            Node::Select {
                id,
                position: _,
                input: _,
                columns,
                outputs: _
            } => {
                if let Some(input) = inputs.pop() {
                    // Create frame.
                    let exprs = columns.iter().map(|column| {
                        let mut expr = col(&*column.column);
                        if !column.alias.is_empty() {
                            expr = expr.alias(&*column.alias);
                        }
                        expr
                    })
                        .collect::<Vec<_>>();
                    let frame = input.frame.select(&exprs);

                    // Create columns.
                    let mut schema: HashMap::<String, DataType> = HashMap::new();
                    columns
                        .iter()
                        .for_each(|column| {
                            if let Some(dtype) = input.schema.get(&column.column) {
                                let name = if column.alias.is_empty() {
                                    column.column.clone()
                                } else {
                                    column.alias.clone()
                                };
                                schema.insert(name, dtype.clone());
                            } else {
                                // TODO: Error: Column not found!
                                todo!("Column not found");
                            }
                        });

                    let query = PolarsQuery {
                        frame,
                        schema: Arc::new(schema)
                    };
                    Ok(query)
                } else {
                    Err(PoldaError::QueryError(format!("Node {} is missing an input table", id)))
                }
            }

            _ => todo!()
        }
    }

    pub fn new(frame: LazyFrame, schema: Arc<Schema>) -> PolarsQuery {
        PolarsQuery { frame, schema }
    }
}
