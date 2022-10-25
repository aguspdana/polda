use polars::frame::DataFrame;

use crate::{error::PoldaError, node::Node};

#[derive(Clone)]
pub struct DuckDbQuery {
    pub query: Vec<(String, String)>,
    pub path: String
}

impl DuckDbQuery {
    pub fn collect(&self) -> Result<DataFrame, PoldaError> {
        todo!()
    }

    pub fn new(node: &Node, inputs: Vec<DuckDbQuery>) -> Result<DuckDbQuery, PoldaError> {
        todo!()
    }

    pub fn same_backend(&self, other: &DuckDbQuery) -> bool {
        self.path == other.path
    }
}
