use crate::error::QueryError;
use crate::node::Position;

mod operation;

pub use operation::UnionNodeOperation;

#[derive(Debug, Clone)]
pub struct UnionNode {
    first_input: Option<String>,
    second_input: Option<String>,
    outputs: Vec<String>,
    position: Position
}

impl UnionNode {
    pub fn execute_operation(&mut self, operation: UnionNodeOperation) -> Result<UnionNodeOperation, QueryError> {
        match operation {
            UnionNodeOperation::SetPosition { position } => {
                let undo = UnionNodeOperation::SetPosition {
                    position: self.position.clone()
                };
                self.position = position;
                Ok(undo)
            }

            UnionNodeOperation::SetFirstInput { input } => {
                let undo = UnionNodeOperation::SetFirstInput {
                    input: self.first_input.clone()
                };
                self.first_input = input;
                Ok(undo)
            }

            UnionNodeOperation::SetSecondInput { input } => {
                let undo = UnionNodeOperation::SetSecondInput {
                    input: self.second_input.clone()
                };
                self.second_input = input;
                Ok(undo)
            }
        }
    }

    pub fn inputs(&self) -> Vec<String> {
        let mut inputs = vec![];
        if let Some(input) = &self.first_input {
            inputs.push(input.clone());
        }
        if let Some(input) = &self.second_input {
            inputs.push(input.clone());
        }
        inputs
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

    pub fn reset_input_operations(&self, id: &String) -> Vec<UnionNodeOperation> {
        let mut ops = vec![];
        if Some(id) == self.first_input.as_ref() {
            ops.push(UnionNodeOperation::SetFirstInput { input: None });
        }
        if Some(id) == self.second_input.as_ref() {
            ops.push(UnionNodeOperation::SetSecondInput { input: None });
        }
        ops
    }
}
