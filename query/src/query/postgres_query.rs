use polars::frame::DataFrame;

use crate::error::PoldaError;
use crate::node::Node;

#[derive(Clone)]
pub struct PostgresQuery {
    pub query: Vec<(String, String)>,
    pub uri: String
}

impl PostgresQuery {
    pub fn collect(self) -> Result<DataFrame, PoldaError> {
        todo!()
    }

    pub fn new(node: &Node, inputs: Vec<PostgresQuery>) -> Result<PostgresQuery, PoldaError> {
        todo!()
    }

    pub fn same_backend(&self, other: &PostgresQuery) -> bool {
        self.uri == other.uri
    }
}
