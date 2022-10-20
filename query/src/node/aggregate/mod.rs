use crate::error::QueryError;
use crate::node::Position;

mod aggregate;
mod computation;
mod operation;

pub use aggregate::Aggregate;
pub use computation::AggregateComputation;
pub use operation::AggregateNodeOperation;

#[derive(Debug, Clone)]
pub struct AggregateNode {
    input: Option<String>,
    outputs: Vec<String>,
    aggregates: Vec<Aggregate>,
    position: Position
}

impl AggregateNode {
    /// Execute an operation and return the undo operation.
    pub fn execute_operation(&mut self, operation: AggregateNodeOperation) -> Result<AggregateNodeOperation, QueryError> {
        match operation {
            AggregateNodeOperation::SetInput { input } => {
                let undo = AggregateNodeOperation::SetInput {
                    input: self.input.clone()
                };
                self.input = input;
                Ok(undo)
            }
            AggregateNodeOperation::SetPosition { position } => {
                let undo = AggregateNodeOperation::SetPosition {
                    position: self.position.clone()
                };
                self.position = position;
                Ok(undo)
            }
            AggregateNodeOperation::InsertAggregate { index, aggregate } => {
                if self.aggregates.len() < index {
                    Err(QueryError::Unsyncable)
                } else {
                    self.aggregates.splice(index..index, [aggregate]);
                    let undo = AggregateNodeOperation::DeleteAggregate {
                        index
                    };
                    Ok(undo)
                }
            }
            AggregateNodeOperation::DeleteAggregate { index } => {
                if index >= self.aggregates.len() {
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = AggregateNodeOperation::InsertAggregate {
                        index,
                        aggregate: self.aggregates[index].clone()
                    };
                    let end = index + 1;
                    self.aggregates.splice(index..end, []);
                    Ok(undo)
                }
            }
            AggregateNodeOperation::SetComputation { index, computation } => {
                if self.aggregates.len() <= index {
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = AggregateNodeOperation::SetComputation {
                        index,
                        computation: self.aggregates[index].computation.clone()
                    };
                    self.aggregates[index].computation = computation;
                    Ok(undo)
                }
            }
            AggregateNodeOperation::SetColumn { index, column } => {
                if self.aggregates.len() <= index {
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = AggregateNodeOperation::SetColumn {
                        index,
                        column: self.aggregates[index].alias.clone()
                    };
                    self.aggregates[index].column = column;
                    Ok(undo)
                }
            }
            AggregateNodeOperation::SetAlias { index, alias } => {
                if self.aggregates.len() <= index {
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = AggregateNodeOperation::SetAlias {
                        index,
                        alias: self.aggregates[index].alias.clone()
                    };
                    self.aggregates[index].alias = alias;
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

    pub fn reset_input_operations(&self, id: &String) -> Vec<AggregateNodeOperation> {
        if Some(id) == self.input.as_ref() {
            vec![AggregateNodeOperation::SetInput { input: None }]
        } else {
            vec![]
        }
    }
}
