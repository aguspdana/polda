use crate::error::PoldaError;
use crate::node::Position;

mod column;
mod join_type;
mod operation;

pub use column::JoinColumn;
pub use join_type::JoinType;
pub use operation::JoinNodeOperation;

#[derive(Debug, Clone)]
pub struct JoinNode {
    left_input: Option<String>,
    right_input: Option<String>,
    outputs: Vec<String>,
    join_type: JoinType,
    columns: Vec<JoinColumn>,
    position: Position,
}

impl JoinNode {
    pub fn execute_operation(&mut self, operation: JoinNodeOperation) -> Result<JoinNodeOperation, PoldaError> {
        match operation {
            JoinNodeOperation::SetPosition { position } => {
                let undo = JoinNodeOperation::SetPosition {
                    position: self.position.clone()
                };
                self.position = position;
                Ok(undo)
            }

            JoinNodeOperation::SetLeftInput { input } => {
                let undo = JoinNodeOperation::SetLeftInput {
                    input: self.left_input.clone()
                };
                self.left_input = input;
                Ok(undo)
            }

            JoinNodeOperation::SetRightInput { input } => {
                let undo = JoinNodeOperation::SetRightInput {
                    input: self.right_input.clone()
                };
                self.right_input = input;
                Ok(undo)
            }

            JoinNodeOperation::SetJoinType { join_type } => {
                let undo = JoinNodeOperation::SetJoinType {
                    join_type: self.join_type.clone()
                };
                self.join_type = join_type;
                Ok(undo)
            }

            JoinNodeOperation::InsertJoinColumn { index, column } => {
                if index > self.columns.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = JoinNodeOperation::DeleteJoinColumn {
                        index
                    };
                    self.columns.splice(index..index, [column]);
                    Ok(undo)
                }
            }

            JoinNodeOperation::DeleteJoinColumn { index } => {
                if index >= self.columns.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = JoinNodeOperation::InsertJoinColumn {
                        index,
                        column: self.columns[index].clone()
                    };
                    let end = index + 1;
                    self.columns.splice(index..end, []);
                    Ok(undo)
                }
            }

            JoinNodeOperation::SetLeftColumn { index, column } => {
                if index >= self.columns.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = JoinNodeOperation::SetLeftColumn {
                        index,
                        column: self.columns[index].left.clone()
                    };
                    self.columns[index].left = column;
                    Ok(undo)
                }
            }

            JoinNodeOperation::SetRightColumn { index, column } => {
                if index >= self.columns.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = JoinNodeOperation::SetRightColumn {
                        index,
                        column: self.columns[index].right.clone()
                    };
                    self.columns[index].right = column;
                    Ok(undo)
                }
            }
        }
    }

    pub fn inputs(&self) -> Vec<String> {
        let mut inputs = vec![];
        if let Some(input) = &self.left_input {
            inputs.push(input.clone());
        }
        if let Some(input) = &self.right_input {
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

    pub fn reset_input_operations(&self, id: &String) -> Vec<JoinNodeOperation> {
        let mut ops = vec![];
        if Some(id) == self.left_input.as_ref() {
            ops.push(JoinNodeOperation::SetLeftInput { input: None });
        }
        if Some(id) == self.right_input.as_ref() {
            ops.push(JoinNodeOperation::SetRightInput { input: None });
        }
        ops
    }
}
