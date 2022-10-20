use crate::error::QueryError;
use crate::node::Position;

mod column;
mod operation;

pub use column::SelectColumn;
pub use operation::SelectNodeOperation;

#[derive(Debug, Clone)]
pub struct SelectNode {
    input: Option<String>,
    outputs: Vec<String>,
    columns: Vec<SelectColumn>,
    position: Position
}

impl SelectNode {
    pub fn execute_operation(&mut self, operation: SelectNodeOperation) -> Result<SelectNodeOperation, QueryError> {
        match operation {
            SelectNodeOperation::SetPosition { position } => {
                let undo = SelectNodeOperation::SetPosition {
                    position: self.position.clone()
                };
                self.position = position;
                Ok(undo)
            }

            SelectNodeOperation::SetInput { input } => {
                let undo = SelectNodeOperation::SetInput {
                    input: self.input.clone()
                };
                self.input = input;
                Ok(undo)
            }

            SelectNodeOperation::InsertColumn { index, column } => {
                if index > self.columns.len() {
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = SelectNodeOperation::DeleteColumn {
                        index
                    };
                    self.columns.splice(index..index, [column]);
                    Ok(undo)
                }
            }

            SelectNodeOperation::DeleteColumn { index } => {
                if index >= self.columns.len() {
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = SelectNodeOperation::InsertColumn {
                        index,
                        column: self.columns[index].clone()
                    };
                    let end = index + 1;
                    self.columns.splice(index..end, []);
                    Ok(undo)
                }
            }

            SelectNodeOperation::SetColumn { index, column } => {
                if index >= self.columns.len() {
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = SelectNodeOperation::SetColumn {
                        index,
                        column: self.columns[index].column.clone()
                    };
                    self.columns[index].column = column;
                    Ok(undo)
                }
            }

            SelectNodeOperation::SetAlias { index, alias } => {
                if index >= self.columns.len() {
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = SelectNodeOperation::SetAlias {
                        index,
                        alias: self.columns[index].alias.clone()
                    };
                    self.columns[index].alias = alias;
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

    pub fn reset_input_operations(&self, id: &String) -> Vec<SelectNodeOperation> {
        if Some(id) == self.input.as_ref() {
            vec![SelectNodeOperation::SetInput { input: None }]
        } else {
            vec![]
        }
    }
}
