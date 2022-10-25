use crate::error::PoldaError;
use crate::node::Position;

mod direction;
mod operation;
mod sorter;

pub use direction::SortDirection;
pub use operation::SortNodeOperation;
pub use sorter::Sorter;

#[derive(Debug, Clone)]
pub struct SortNode {
    input: Option<String>,
    outputs: Vec<String>,
    sorters: Vec<Sorter>,
    position: Position
}

impl SortNode {
    pub fn execute_operation(&mut self, operation: SortNodeOperation) -> Result<SortNodeOperation, PoldaError> {
        match operation {
            SortNodeOperation::SetPosition { position } => {
                let undo = SortNodeOperation::SetPosition {
                    position: self.position.clone()
                };
                self.position = position;
                Ok(undo)
            }

            SortNodeOperation::SetInput { input } => {
                let undo = SortNodeOperation::SetInput {
                    input: self.input.clone()
                };
                self.input = input;
                Ok(undo)
            }

            SortNodeOperation::InsertSorter { index, sorter } => {
                if index > self.sorters.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = SortNodeOperation::DeleteSorter { index };
                    self.sorters.splice(index..index, [sorter]);
                    Ok(undo)
                }
            }

            SortNodeOperation::DeleteSorter { index } => {
                if index >= self.sorters.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = SortNodeOperation::InsertSorter {
                        index,
                        sorter: self.sorters[index].clone()
                    };
                    let end = index + 1;
                    self.sorters.splice(index..end, []);
                    Ok(undo)
                }
            }

            SortNodeOperation::SetColumn { index, column } => {
                if index >= self.sorters.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = SortNodeOperation::SetColumn {
                        index,
                        column: self.sorters[index].column.clone()
                    };
                    self.sorters[index].column = column;
                    Ok(undo)
                }
            }

            SortNodeOperation::SetDirection { index, direction } => {
                if index >= self.sorters.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = SortNodeOperation::SetDirection {
                        index,
                        direction: self.sorters[index].direction.clone()
                    };
                    self.sorters[index].direction = direction;
                    Ok(undo)
                }
            }
        }
    }

    pub fn inputs(&self) -> Vec<String> {
        if let Some(input) = &self.input {
            vec![input.clone()]
        } else {
            vec![]
        }
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

    pub fn reset_input_operations(&self, id: &String) -> Vec<SortNodeOperation> {
        if Some(id) == self.input.as_ref() {
            vec![SortNodeOperation::SetInput { input: None }]
        } else {
            vec![]
        }
    }
}
