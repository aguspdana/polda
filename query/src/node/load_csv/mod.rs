use crate::error::QueryError;
use crate::node::Position;

mod operation;

pub use operation::LoadCsvNodeOperation;

#[derive(Debug, Clone)]
pub struct LoadCsvNode {
    outputs: Vec<String>,
    path: String,
    position: Position
}

impl LoadCsvNode {
    pub fn execute_operation(&mut self, operation: LoadCsvNodeOperation) -> Result<LoadCsvNodeOperation, QueryError> {
        match operation {
            LoadCsvNodeOperation::SetPosition { position } => {
                let undo = LoadCsvNodeOperation::SetPosition {
                    position: self.position.clone()
                };
                self.position = position;
                Ok(undo)
            }

            LoadCsvNodeOperation::SetPath { path } => {
                let undo = LoadCsvNodeOperation::SetPath {
                    path: self.path.clone()
                };
                self.path = path;
                Ok(undo)
            }
        }
    }

    pub fn inputs(&self) -> Vec<String> {
        vec![]
    }

    pub fn insert_output(&mut self, output: String) {
        if !self.outputs.contains(&output) {
            self.outputs.push(output);
        }
    }

    pub fn outputs(&self) -> Vec<String> {
        self.outputs
            .iter()
            .map(|id| id.clone())
            .collect()
    }

    pub fn remove_output(&mut self, output: &String) {
        if self.outputs.contains(output) {
            self.outputs = self.outputs
                .drain(..)
                .filter(|id| id != output)
                .collect();
        }
    }

    pub fn reset_input_operations(&self, _id: &String) -> Vec<LoadCsvNodeOperation> {
        vec![]
    }
}
