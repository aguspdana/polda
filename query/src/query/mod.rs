use polars::frame::DataFrame;
use polars::prelude::Field;
use polars::prelude::IntoLazy;
use std::collections::HashMap;
use std::sync::Arc;

use crate::data_type::DataType;

mod duck_db_query;
mod polars_query;
mod schema;
mod types;

pub use duck_db_query::DuckDbQuery;
pub use polars_query::PolarsQuery;
pub use schema::Schema;
pub use types::SqlQuery;

use crate::error::PoldaError;
use crate::doc::Node;

#[derive(Clone)]
pub enum Query {
    Polars(PolarsQuery),
    DuckDb(DuckDbQuery)
}

impl Query {
    pub fn collect(self) -> Result<DataFrame, PoldaError> {
        use Query::*;
        match self {
            DuckDb(q) => q.collect(),
            Polars(q) => q.collect(),
        }
    }

    /// Unwrap `DuckDbQuery`. Error if it's a not duck db query.
    pub fn duck_db(self) -> Result<DuckDbQuery, PoldaError> {
        if let Query::DuckDb(q) = self {
            Ok(q)
        } else {
            Err(PoldaError::QueryError(format!("Can't unwrap query into DuckDbQuery")))
        }
    }

    pub fn from_node(node: &Node, inputs: Vec<Query>) -> Result<Query, PoldaError> {
        use Node::*;
        match node {
            LoadCsv { id: _, position: _, filename: _, outputs: _ } => {
                PolarsQuery::from_node(node, vec![])
                    .map(|q| Query::Polars(q))
            }

            // Nodes that require input table(s).
            _ => {
                if let Some(first) = inputs.first() {
                    match first {
                        Query::DuckDb(_) => {
                            let mut duck_inputs = Vec::with_capacity(inputs.len());
                            for input in inputs {
                                duck_inputs.push(input.duck_db()?);
                            }
                            DuckDbQuery::from_node(node, duck_inputs)
                                .map(|q| Query::DuckDb(q))
                        }

                        Query::Polars(_) => {
                            let mut polars_inputs = Vec::with_capacity(inputs.len());
                            for input in inputs {
                                polars_inputs.push(input.polars()?);
                            }
                            PolarsQuery::from_node(node, polars_inputs)
                                .map(|q| Query::Polars(q))
                        }
                    }
                } else {
                    // Missing inputs!
                    Err(PoldaError::QueryError(format!("Node \"{}\" is missing input node", node.id())))
                }
            }
        }
    }

    /// Unwrap `PolarsQuery`.  If it's not a Polars query, it'll be
    /// materialized first.
    pub fn polars(self) -> Result<PolarsQuery, PoldaError> {
        use Query::*;
        match self {
            Polars(q) => Ok(q),
            other => {
                let frame = other.collect()?;
                let mut schema = HashMap::new();
                for field in frame.schema().iter_fields() {
                    let Field { name, dtype } = field;
                    let dtype = DataType::try_from(dtype)?;
                    schema.insert(name, dtype);
                }
                let query = PolarsQuery {
                    frame: IntoLazy::lazy(frame),
                    schema: Schema(Arc::new(schema))
                };
                Ok(query)
            }
        }
    }

    pub fn same_backend(&self, other: &Query) -> bool {
        use Query::*;
        match (self, other) {
            (DuckDb(q1), DuckDb(q2)) => q1.same_backend(q2),
            (Polars(_), Polars(_)) => true,
            _ => false
        }
    }
}
