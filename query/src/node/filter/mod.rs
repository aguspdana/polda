use crate::error::PoldaError;
use crate::node::Position;

mod filter;
mod operation;
mod predicate;

pub use filter::Filter;
pub use operation::FilterNodeOperation;
pub use predicate::FilterPredicate;

#[derive(Debug, Clone)]
pub struct FilterNode {
    input: Option<String>,
    outputs: Vec<String>,
    filters: Vec<Filter>,
    position: Position
}

impl FilterNode {
    pub fn execute_operation(&mut self, operation: FilterNodeOperation) -> Result<FilterNodeOperation, PoldaError> {
        match operation {
            FilterNodeOperation::SetPosition { position } => {
                let undo = FilterNodeOperation::SetPosition {
                    position: self.position.clone()
                };
                self.position = position;
                Ok(undo)
            }

            FilterNodeOperation::SetInput { input } => {
                let undo = FilterNodeOperation::SetInput {
                    input: self.input.clone()
                };
                self.input = input;
                Ok(undo)
            }

            FilterNodeOperation::InsertFilter { index, filter } => {
                if index > self.filters.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = FilterNodeOperation::DeleteFilter { index };
                    self.filters.splice(index..index, [filter]);
                    Ok(undo)
                }
            }

            FilterNodeOperation::DeleteFilter { index } => {
                if index >= self.filters.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = FilterNodeOperation::InsertFilter {
                        index,
                        filter: self.filters[index].clone()
                    };
                    let end = index + 1;
                    self.filters.splice(index..end, []);
                    Ok(undo)
                }
            }

            FilterNodeOperation::SetColumn { index, column } => {
                if index >= self.filters.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = FilterNodeOperation::SetColumn {
                        index,
                        column: self.filters[index].column.clone()
                    };
                    self.filters[index].column = column;
                    Ok(undo)
                }
            }

            FilterNodeOperation::SetPredicate { index, predicate } => {
                if index >= self.filters.len() {
                    Err(PoldaError::Unsyncable)
                } else {
                    let undo = FilterNodeOperation::SetPredicate {
                        index,
                        predicate: self.filters[index].predicate.clone()
                    };
                    self.filters[index].predicate = predicate;
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

    pub fn reset_input_operations(&self, id: &String) -> Vec<FilterNodeOperation> {
        if Some(id) == self.input.as_ref() {
            vec![FilterNodeOperation::SetInput { input: None }]
        } else {
            vec![]
        }
    }
}
