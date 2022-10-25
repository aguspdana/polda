use polars::frame::DataFrame;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::column::Column;
use crate::error::PoldaError;
use crate::node::Node;
use crate::query::Query;

mod operation;

pub use operation::DocOperation;

pub struct Doc {
    nodes: HashMap<String, Node>,
    index: Vec<String>
}

impl Doc {
    pub fn collect(&self, id: &String) -> Result<DataFrame, PoldaError> {
        // TODO: Validate the query before executing.

        let mut queries: HashMap<String, Query> = HashMap::new();
        let mut polars_queries: HashMap<String, Query> = HashMap::new();
        let mut nodes_to_query = vec![id.clone()];

        while let Some(id) = nodes_to_query.last().cloned() {
            if let Some(node) = self.nodes.get(&id) {
                let inputs = node.inputs();

                // Ensure all inputs are in queries, otherwise push to
                // nodes_to_query.
                let mut are_inputs_in_queries = true;
                inputs.iter().for_each(|id| {
                    if !queries.contains_key(id) {
                        are_inputs_in_queries = false;
                        nodes_to_query.push(id.clone());
                    }
                });

                if !are_inputs_in_queries {
                    continue;
                }

                // Check if the input queries have the same backend.
                let mut same_backend = true;
                if let Some(first_input) = inputs.first() {
                    if let Some(first_input_query) = queries.get(first_input) {
                        for input in inputs[1..].iter() {
                            let input_query = queries.get(input).unwrap();
                            if !first_input_query.same_backend(input_query) {
                                same_backend = false;
                                break;
                            }
                        }
                    }
                }

                let inputs_queries: Vec<Query> = if !same_backend {
                    // Input queries have different backends: Convert them into
                    // polars queries.
                    for input in inputs.iter() {
                        if !polars_queries.contains_key(input) {
                            let query = queries
                                .get(input)
                                .unwrap()
                                .clone()
                                .polars()?;
                            let query = Query::Polars(query);
                            polars_queries.insert(input.clone(), query);
                        }
                    }

                    inputs
                        .iter()
                        .map(|input| {
                            polars_queries.get(input).unwrap().clone()
                        })
                        .collect()
                } else {
                    // Input queries have the same backend: Build the query for
                    // current node.
                    inputs
                        .iter()
                        .map(|input| {
                            queries.get(input).unwrap().clone()
                        })
                        .collect()
                };

                let query = Query::new(node, inputs_queries)?;
                queries.insert(id.clone(), query);
                nodes_to_query.pop();
            } else {
                // TODO: Error
            }
        }

        queries.remove(id).unwrap().collect()
    }

    /// Execute operations and return operations (if any) that used to break
    /// cycles in the graph.
    pub fn execute_opearations(&mut self, operations: Vec<DocOperation>) -> Result<Vec<DocOperation>, PoldaError> {
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
    pub fn execute_operation(&mut self, operation: DocOperation) -> Result<DocOperation, PoldaError> {
        match operation {
            DocOperation::InsertNode { id, node } => {
                if self.nodes.contains_key(&id) {
                    Err(PoldaError::Unsyncable)
                } else if node.inputs().len() != 0 || node.outputs().len() != 0 {
                    // Disallow inserting a node that already has inputs/outputs.
                    Err(PoldaError::Unsyncable)
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
                        Err(PoldaError::Unsyncable)
                    } else {
                        self.nodes
                            .remove(&id)
                            .map(|node| DocOperation::InsertNode { id, node })
                            .ok_or(PoldaError::Unsyncable)
                    }
                } else {
                    Err(PoldaError::Unsyncable)
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
                    Err(PoldaError::Unsyncable)
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
                    Err(PoldaError::Unsyncable)
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
                        Err(PoldaError::Unsyncable)
                    }
                } else {
                    Err(PoldaError::Unsyncable)
                }
            }
        }
    }

    pub fn new() -> Doc {
        Doc {
            nodes: HashMap::new(),
            index: vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{LoadCsvNode, Position};
    use super::*;

    #[test]
    fn doc_collect() {
        let mut doc = Doc::new();
        let ops = vec![
            DocOperation::InsertNode {
                id: String::from("a"),
                node: Node::LoadCsv(LoadCsvNode {
                    outputs: vec![],
                    path: String::from("data/test.csv"),
                    position: Position {
                        x: 0,
                        y: 0
                    }
                })
            },
            DocOperation::InsertIndex {
                index: 0,
                id: String::from("a")
            }
        ];
        doc.execute_opearations(ops).unwrap();
        println!("{}", doc.collect(&String::from("a")).unwrap());
    }
}
