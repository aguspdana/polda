use polars::frame::DataFrame;
use polars::prelude::IntoLazy;
use crate::error::PoldaError;
use crate::node::Node;

mod duck_db_query;
mod polars_query;
mod postgres_query;

pub use duck_db_query::DuckDbQuery;
pub use polars_query::PolarsQuery;
pub use postgres_query::PostgresQuery;

#[derive(Clone)]
pub enum Query {
    DuckDb(DuckDbQuery),
    Polars(PolarsQuery),
    Postgres(PostgresQuery)
}

impl Query {
    pub fn collect(self) -> Result<DataFrame, PoldaError> {
        match self {
            Query::DuckDb(q) => q.collect(),
            Query::Polars(q) => q.collect(),
            Query::Postgres(q) => q.collect()
        }
    }

    /// Unwrap `DuckDbQuery`. Error if it's a not duck db query.
    pub fn duck_db(self) -> Result<DuckDbQuery, PoldaError> {
        if let Query::DuckDb(q) = self {
            Ok(q)
        } else {
            Err(PoldaError::QueryError)
        }
    }

    /// Create a query from a node.  Error if the input queries must have
    /// different backends.
    pub fn new(node: &Node, inputs: Vec<Query>) -> Result<Query, PoldaError> {
        match node {
            Node::LoadCsv(_) => {
                PolarsQuery::new(node, vec![])
                    .map(|q| Query::Polars(q))
            }

            _ => {
                if let Some(first) = inputs.first() {
                    match first {
                        Query::DuckDb(_) => {
                            let mut duck_inputs = Vec::with_capacity(inputs.len());
                            for input in inputs {
                                duck_inputs.push(input.duck_db()?);
                            }
                            DuckDbQuery::new(node, duck_inputs)
                                .map(|q| Query::DuckDb(q))
                        }

                        Query::Polars(_) => {
                            let mut polars_inputs = Vec::with_capacity(inputs.len());
                            for input in inputs {
                                polars_inputs.push(input.polars()?);
                            }
                            PolarsQuery::new(node, polars_inputs)
                                .map(|q| Query::Polars(q))
                        }

                        Query::Postgres(_) => {
                            let mut postgres_inputs = Vec::with_capacity(inputs.len());
                            for input in inputs {
                                postgres_inputs.push(input.postgres()?);
                            }
                            PostgresQuery::new(node, postgres_inputs)
                                .map(|q| Query::Postgres(q))
                        }
                    }
                } else {
                    // Missing inputs!
                    Err(PoldaError::QueryError)
                }
            }
        }
    }

    /// Unwrap `PolarsQuery`.  If it's not a polars query, it'll be
    /// materialized first.
    pub fn polars(self) -> Result<PolarsQuery, PoldaError> {
        match self {
            Query::Polars(q) => Ok(q),
            other => {
                let df = other.collect()?;
                let lazy = IntoLazy::lazy(df);
                Ok(PolarsQuery { query: lazy })
            }
        }
    }

    /// Unwrap `PostgresQuery`.  Error if it's not a postgres query.
    pub fn postgres(self) -> Result<PostgresQuery, PoldaError> {
        if let Query::Postgres(q) = self {
            Ok(q)
        } else {
            Err(PoldaError::QueryError)
        }
    }

    pub fn same_backend(&self, other: &Query) -> bool {
        match (self, other) {
            (Query::DuckDb(q1), Query::DuckDb(q2)) => q1.same_backend(q2),
            (Query::Polars(_), Query::Polars(_)) => true,
            (Query::Postgres(q1), Query::Postgres(q2)) => q1.same_backend(q2),
            _ => false
        }
    }
}
