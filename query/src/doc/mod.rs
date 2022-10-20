use polars::frame::DataFrame;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::column::Column;
use crate::error::QueryError;
use crate::node::Node;

mod operation;

pub use operation::DocOperation;

pub struct Doc {
    nodes: HashMap<String, Node>,
    index: Vec<String>
}

impl Doc {
    pub fn collect(&self, id: &String) -> Result<DataFrame, QueryError> {
        todo!()
    }

    /// Execute operations and return operations (if any) that used to break
    /// cycles in the graph.
    pub fn execute_opearations(&mut self, operations: Vec<DocOperation>) -> Result<Vec<DocOperation>, QueryError> {
        // Get inputs of the altered nodes.
        let mut inputs_before = HashMap::new();
        operations.iter().for_each(|op| {
            if let DocOperation::AlterNode { id, operation: _ } = op {
                if !inputs_before.contains_key(id) {
                    if let Some(node) = self.nodes.get(id) {
                        inputs_before.insert(id.clone(), node.inputs());
                    }
                }
            }
        });

        let mut undo_operations = Vec::with_capacity(operations.len());

        for operation in operations.into_iter().rev() {
            match self.execute_operation(operation) {
                Ok(undo) => undo_operations.push(undo),
                Err(e) => {
                    for undo in undo_operations.into_iter().rev() {
                        self.execute_operation(undo)
                            .expect("Failed to undo operations");
                    }
                    return Err(e);
                }
            }
        }

        // Break cycles if any.
        let mut break_cycle_ops = vec![];
        inputs_before.into_iter().for_each(|(id, inputs)| {
            if let Some(node) = self.nodes.get(&id) {
                let inputs_after = node.inputs();
                inputs_after.into_iter().for_each(|i| {
                    if !inputs.contains(&i) {
                        // Check cycle.
                        let mut checked = HashSet::new();
                        let mut nodes_to_check = vec![i.clone()];
                        while let Some(j) = nodes_to_check.pop() {
                            if checked.contains(&j) {
                                // It's a cycle.  Create operations that break the cycle.
                                node.reset_input_operations(&i).into_iter().for_each(|op| {
                                    break_cycle_ops.push(DocOperation::AlterNode {
                                        id: id.clone(),
                                        operation: op
                                    });
                                });
                            } else if let Some(node) = self.nodes.get(&j) {
                                let mut _inputs = node.inputs();
                                nodes_to_check.append(&mut _inputs);
                            }
                            checked.insert(j);
                        }
                    }
                })
            }
        });
        break_cycle_ops.clone().into_iter().for_each(|op| {
            self.execute_operation(op).expect("BUG: Failed to execute fix operation");
        });

        Ok(break_cycle_ops)
    }

    /// Execute an operation and return the undo operation.  Return `None` if
    /// the operation can't be executed.
    pub fn execute_operation(&mut self, operation: DocOperation) -> Result<DocOperation, QueryError> {
        match operation {
            DocOperation::InsertNode { id, node } => {
                if self.nodes.contains_key(&id) {
                    Err(QueryError::Unsyncable)
                } else if node.inputs().len() != 0 || node.outputs().len() != 0 {
                    // Disallow inserting a node that already has inputs/outputs.
                    Err(QueryError::Unsyncable)
                } else {
                    let undo = DocOperation::DeleteNode { id: id.clone() };
                    self.nodes.insert(id, node);
                    Ok(undo)
                }
            }

            DocOperation::DeleteNode { id } => {
                if let Some(node) = self.nodes.get(&id) {
                    if node.inputs().len() != 0 || node.outputs().len() != 0 {
                        // Disallow inserting a node that still has inputs/outputs.
                        Err(QueryError::Unsyncable)
                    } else {
                        self.nodes
                            .remove(&id)
                            .map(|node| DocOperation::InsertNode { id, node })
                            .ok_or(QueryError::Unsyncable)
                    }
                } else {
                    Err(QueryError::Unsyncable)
                }
            }

            DocOperation::AlterNode { id, operation } => {
                let mut remove_output_from_node = vec![];
                let mut insert_output_to_node = vec![];

                let res = if let Some(node) = self.nodes.get_mut(&id) {
                    let inputs_before = node.inputs();
                    let undo = node.execute_operation(operation)
                        .map(|undo| {
                            DocOperation::AlterNode {
                                id: id.clone(),
                                operation: undo
                            }
                        });
                    let inputs_after = node.inputs();

                    if inputs_before != inputs_after {
                        inputs_before.iter()
                            .for_each(|input| {
                                if !inputs_after.contains(input) {
                                    // Remove id from input node's output.
                                    remove_output_from_node.push(input.clone());
                                }
                            });
                        inputs_after.iter()
                            .for_each(|input| {
                                if !inputs_after.contains(input) {
                                    // Add id to input node's output.
                                    insert_output_to_node.push(input.clone());
                                }
                            });
                    }

                    undo
                } else {
                    Err(QueryError::Unsyncable)
                };

                remove_output_from_node.iter().for_each(|node_id| {
                    if let Some(node) = self.nodes.get_mut(node_id) {
                        node.remove_output(&id);
                    }
                });

                insert_output_to_node.iter().for_each(|node_id| {
                    if let Some(node) = self.nodes.get_mut(node_id) {
                        node.insert_output(id.clone());
                    }
                });

                res
            }

            DocOperation::InsertIndex { index, id } => {
                if self.index.len() >= index {
                    let undo = DocOperation::DeleteIndex {
                        index,
                        id: id.clone()
                    };
                    self.index.splice(index..index, [id]);
                    Ok(undo)
                } else {
                    Err(QueryError::Unsyncable)
                }
            }

            DocOperation::DeleteIndex { index, id } => {
                if self.index.len() > index {
                    if &self.index[index] == &id {
                        let undo = DocOperation::InsertIndex {
                            index,
                            id: id.clone()
                        };
                        let end = index + 1;
                        self.index.splice(index..end, []);
                        Ok(undo)
                    } else {
                        Err(QueryError::Unsyncable)
                    }
                } else {
                    Err(QueryError::Unsyncable)
                }
            }
        }
    }

    pub fn columns(&self, id: &String) -> Vec<Column> {
        todo!()
    }
}
