use polars::frame::DataFrame;
use polars::lazy::prelude::col;
use polars::lazy::frame::LazyFrame;
use polars::prelude::LazyCsvReader;

use crate::error::PoldaError;
use crate::node::Node;

#[derive(Clone)]
pub struct PolarsQuery {
    pub query: LazyFrame
}

impl PolarsQuery {
    pub fn collect(self) -> Result<DataFrame, PoldaError> {
        Ok(self.query.collect()?)
    }

    pub fn new(node: &Node, mut inputs: Vec<PolarsQuery>) -> Result<PolarsQuery, PoldaError> {
        match node {
            Node::LoadCsv(node) => {
                let query = LazyCsvReader::new(&node.path).finish()?;
                Ok(PolarsQuery { query })
            }

            Node::Select(node) => {
                if let Some(input) = inputs.pop() {
                    let cols = node.columns.iter().map(|column| {
                        let mut expr = col(&*column.column);
                        if !column.alias.is_empty() {
                            expr = expr.alias(&*column.alias);
                        }
                        expr
                    }).collect::<Vec<_>>();
                    let query = input.query.select(&cols);
                    Ok(PolarsQuery { query })
                } else {
                    Err(PoldaError::QueryError)
                }
            }

            _ => todo!()
        }
    }
}
