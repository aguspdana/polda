use polars::frame::DataFrame;
use std::sync::Arc;

use crate::doc::Node;
use crate::error::PoldaError;
use super::Schema;
use super::SqlQuery;

#[derive(Debug, Clone)]
pub struct DuckDbQuery {
    query: Vec<Arc<SqlQuery>>,
    path: Arc<String>,
    schema: Arc<Schema>
}

impl DuckDbQuery {
    pub fn collect(self) -> Result<DataFrame, PoldaError> {
        todo!()
    }

    pub fn from_node(
        _node: &Node,
        mut _inputs: Vec<DuckDbQuery>
    ) -> Result<DuckDbQuery, PoldaError> {
        todo!()
    }

    pub fn new(
        query: Vec<Arc<SqlQuery>>,
        path: Arc<String>,
        schema: Arc<Schema>
    ) -> DuckDbQuery {
        DuckDbQuery { query, path, schema }
    }

    pub fn path(&self) -> Arc<String> {
        self.path.clone()
    }

    pub fn query(&self) -> &Vec<Arc<SqlQuery>> {
        &self.query
    }

    pub fn same_backend(&self, other: &DuckDbQuery) -> bool {
        self.path == other.path
    }

    pub fn schema(&self) -> Arc<Schema> {
        self.schema.clone()
    }
}
