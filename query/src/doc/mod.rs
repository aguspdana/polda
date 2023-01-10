// TODO: We need to store execution context like project directory and remote
// sources.
use polars::frame::DataFrame;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::error::PoldaError;
use crate::query::Query;

mod node;
mod operation;
mod types;

pub use node::Node;
pub use operation::Operation;
pub use operation::transform_batch;
pub use operation::validate_sequence;
pub use types::aggregate::Aggregate;
pub use types::aggregate::AggregateComputation;
pub use types::case::Case;
pub use types::compute::ComputeOperation;
pub use types::filter::FilterPredicate;
pub use types::join::JoinType;
pub use types::join::JoinColumn;
pub use types::select::SelectColumn;
pub use types::sort::Sorter;
pub use types::sort::SortDirection;
pub use types::InputName;
pub use types::InputPort;
pub use types::Position;
pub use types::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doc {
    nodes: HashMap<String, Node>,
    index: Vec<String>
}

impl Doc {
    pub fn collect(&self, id: &String, limit: Option<usize>) -> Result<DataFrame, PoldaError> {
        collect(&self.nodes, id, limit)
    }

    /// Get a node and it's dependecies.
    pub fn extract_nodes(&self, id: &String) -> Result<HashMap<String, Node>, PoldaError> {
        let mut nodes = HashMap::new();
        let mut ids = vec![id.clone()];

        while let Some(id) = ids.pop() {
            if let Some(node) = self.nodes.get(&id) {
                let inputs = node.inputs();
                for input in inputs.iter() {
                    if let Some(input) = input {
                        ids.push(input.clone());
                    } else {
                        return Err(PoldaError::QueryError(format!("Node with id \"{}\" is missing an input", id)));
                    }
                }
                nodes.insert(id, node.clone());
            } else {
                return Err(PoldaError::DocError(format!("Node with id \"{}\" doesn't exist", id)));
            }
        }

        Ok(nodes)
    }

    /// Check whether a connection from `from` to `to` create a cycle or not.
    /// Return `Err` if there's any dependency that doesn't exist in the doc.
    pub fn is_cycle(&self, from: &String, to: &String) -> Result<bool, PoldaError> {
        let mut ids = vec![from];
        let mut checked = HashSet::new();

        while let Some(id) = ids.pop() {
            if id == to {
                return Ok(true);
            }
            if let Some(node) = self.nodes.get(id) {
                node.inputs()
                    .iter()
                    .for_each(|input| {
                        if let Some(input) = input {
                            if !checked.contains(input) {
                                checked.insert(input);
                            }
                        }
                    });
            } else {
                return Err(PoldaError::OperationError(format!("Node with id \"{}\" doesn't exist", id)));
            }
        }

        Ok(false)
    }

    /// Execute a batch of operations and return the undo operations.
    pub fn execute_operations(
        &mut self,
        operations: Vec<Operation>
    ) -> Result<Vec<Operation>, PoldaError> {
        let mut undo_ops = vec![];
        for op in operations.into_iter() {
            match self.execute_operation(op) {
                Ok(Some(undo)) => undo_ops.push(undo),
                Err(e) => {
                    undo_ops
                        .into_iter()
                        .rev()
                        .for_each(|undo| {
                            self.execute_operation(undo).ok();
                        });
                    return Err(e);
                }
                _ => ()
            }
        }
        Ok(undo_ops)
    }

    /// Execute an operation.  `SetInput` operation may fail if the input has
    /// been deleted or the new connection create a cycle.  In this case the
    /// execution returns `None`.  Other failures return `Err`.
    pub fn execute_operation(
        &mut self,
        operation: Operation
    ) -> Result<Option<Operation>, PoldaError> {
        use Operation::*;

        match operation {
            InsertNode { node } => {
                for input in node.inputs().iter() {
                    if input.is_some() {
                        return Err(PoldaError::OperationError(format!("Can't insert a node that is already connected")));
                    }
                }
                if node.outputs().len() != 0 {
                    return Err(PoldaError::OperationError(format!("Can't insert a node that is already connected")));
                }
                if self.nodes.contains_key(node.id()) {
                    return Err(PoldaError::OperationError(format!("Can't insert a node with a conflicting id")));
                }
                let undo = Operation::DeleteNode {
                    id: node.id().clone()
                };
                self.nodes.insert(node.id().clone(), node);
                Ok(Some(undo))
            }

            DeleteNode { id } => {
                if let Some(node) = self.nodes.remove(&id) {
                    for input in node.inputs().iter() {
                        if input.is_some() {
                            self.nodes.insert(id, node);
                            return Err(PoldaError::OperationError(format!("Can't delete a node that is still connected")));
                        }
                    }
                    if node.outputs().len() != 0 {
                        self.nodes.insert(id, node);
                        return Err(PoldaError::OperationError(format!("Can't delete a node that is still connected")));
                    }
                    let undo = Operation::InsertNode { node };
                    Ok(Some(undo))
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            InsertIndex { id, index } => {
                if index > self.index.len() {
                    return Err(PoldaError::OperationError(format!("Can't insert a node at index {}. Possible index (0 - {})", index, self.index.len())));
                }
                let undo = Operation::DeleteIndex {
                    index,
                    id: id.clone()
                };
                self.index.splice(index..index, [id]);
                Ok(Some(undo))
            }

            DeleteIndex { id, index } => {
                if index >= self.index.len() {
                    return Err(PoldaError::OperationError(format!("Can't delete a node at index {}, Possible index (0 - {})", index, self.index.len()-1)));
                }
                if &self.index[index] != &id {
                    return Err(PoldaError::OperationError(format!("Mismatch id at index {}. Expected \"{}\", found \"{}\"", index, id, self.index[index])));
                }
                let undo = Operation::InsertIndex {
                    index,
                    id: id.clone()
                };
                let end = index + 1;
                self.index.splice(index..end, []);
                Ok(Some(undo))
            }

            SetInput { id, name, input: new_input } => {
                // Return None if the input node doesn't exist or the new
                // connection create a cycle.
                if let Some(new_input) = new_input.as_ref() {
                    if !self.nodes.contains_key(new_input) {
                        return Ok(None);
                    }
                    if self.is_cycle(&id, new_input)? {
                        return Ok(None);
                    }
                }

                // When the input node doesn't exist (maybe it has been deleted
                // by other client) we set the input field regardless.  We fix
                // "input node doesn't exist" and "operation create a cycle"
                // after the batch operations are executed.

                let mut insert_output = None;
                let mut remove_output = None;

                let result = if let Some(node) = self.nodes.get_mut(&id) {
                    use Node::*;

                    match node {
                        Aggregate {
                            id: _,
                            position: _,
                            input,
                            aggregates: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != input {
                                    insert_output = new_input.clone();
                                    remove_output = input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: input.clone()
                                };
                                *input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                Err(PoldaError::OperationError(format!("Aggregate node doesn't take a secondary input")))
                            }
                        }

                        Bins {
                            id: _,
                            position: _,
                            input,
                            name: _,
                            column: _,
                            lower_bound: _,
                            upper_bound: _,
                            count: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != input {
                                    insert_output = new_input.clone();
                                    remove_output = input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: input.clone()
                                };
                                *input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                Err(PoldaError::OperationError(format!("Bins node doesn't take a secondary input")))
                            }
                        }

                        Case {
                            id: _,
                            position: _,
                            input,
                            name: _,
                            data_type: _,
                            cases: _,
                            default: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != input {
                                    insert_output = new_input.clone();
                                    remove_output = input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: input.clone()
                                };
                                *input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                Err(PoldaError::OperationError(format!("Case node doesn't take a secondary input")))
                            }
                        }

                        Cast {
                            id: _,
                            position: _,
                            input,
                            name: _,
                            column: _,
                            data_type: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != input {
                                    insert_output = new_input.clone();
                                    remove_output = input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: input.clone()
                                };
                                *input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                Err(PoldaError::OperationError(format!("Cast node doesn't take a secondary input")))
                            }
                        }

                        Compute {
                            id: _,
                            position: _,
                            input,
                            name: _,
                            column: _,
                            operation: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != input {
                                    insert_output = new_input.clone();
                                    remove_output = input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: input.clone()
                                };
                                *input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                Err(PoldaError::OperationError(format!("Compute node doesn't take a secondary input")))
                            }
                        }

                        Filter {
                            id: _,
                            position: _,
                            input,
                            column: _,
                            predicate: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != input {
                                    insert_output = new_input.clone();
                                    remove_output = input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: input.clone()
                                };
                                *input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                Err(PoldaError::OperationError(format!("Filter node doesn't take a secondary input")))
                            }
                        }

                        Join {
                            id: _,
                            position: _,
                            left_input,
                            right_input,
                            join_type: _,
                            columns: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != left_input && left_input != right_input {
                                    insert_output = new_input.clone();
                                    remove_output = left_input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: left_input.clone()
                                };
                                *left_input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                if &new_input != right_input && left_input != right_input {
                                    insert_output = new_input.clone();
                                    remove_output = right_input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: right_input.clone()
                                };
                                *right_input = new_input.clone();
                                Ok(Some(undo))
                            }
                        }

                        LoadCsv {
                            id: _,
                            position: _,
                            filename: _,
                            outputs: _
                        } => Err(PoldaError::OperationError(format!("Load Csv node doesn't take an input"))),

                        Select {
                            id: _,
                            position: _,
                            input,
                            columns: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != input {
                                    insert_output = new_input.clone();
                                    remove_output = input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: input.clone()
                                };
                                *input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                Err(PoldaError::OperationError(format!("Select node doesn't take a secondary input")))
                            }
                        }

                        Sort {
                            id: _,
                            position: _,
                            input,
                            sorters: _,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != input {
                                    insert_output = new_input.clone();
                                    remove_output = input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: input.clone()
                                };
                                *input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                Err(PoldaError::OperationError(format!("Sort node doesn't take a secondary input")))
                            }
                        }

                        Union {
                            id: _,
                            position: _,
                            primary_input,
                            secondary_input,
                            outputs: _
                        } => {
                            if let InputName::Primary = &name {
                                if &new_input != primary_input && primary_input != secondary_input {
                                    insert_output = new_input.clone();
                                    remove_output = primary_input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: primary_input.clone()
                                };
                                *primary_input = new_input.clone();
                                Ok(Some(undo))
                            } else {
                                if &new_input != secondary_input && primary_input != secondary_input {
                                    insert_output = new_input.clone();
                                    remove_output = secondary_input.clone();
                                }
                                let undo = Operation::SetInput {
                                    id: id.clone(),
                                    name,
                                    input: secondary_input.clone()
                                };
                                *secondary_input = new_input.clone();
                                Ok(Some(undo))
                            }
                        }
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                };

                // Remove from input node's outputs.
                if let Some(input) = &remove_output {
                    if let Some(node) = self.nodes.get_mut(input) {
                        node.remove_output(&id);
                    }
                }

                // Insert into input node's outputs.
                if let Some(input) = &insert_output {
                    if let Some(node) = self.nodes.get_mut(input) {
                        node.insert_output(id);
                    }
                }

                result
            }

            SetPosition { id, position: new_position } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    use Node::*;

                    macro_rules! set_position {
                        ($id:expr, $position:ident, $new_position:expr) => {{
                            let undo = Operation::SetPosition {
                                id: $id,
                                position: $position.clone()
                            };
                            *$position = $new_position;
                            Ok(Some(undo))
                        }};
                    }

                    match node {
                        Aggregate {
                            id: _,
                            position,
                            input: _,
                            aggregates: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        Bins {
                            id: _,
                            position,
                            input: _,
                            name: _,
                            column: _,
                            lower_bound: _,
                            upper_bound: _,
                            count: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        Case {
                            id: _,
                            position,
                            input: _,
                            name: _,
                            data_type: _,
                            cases: _,
                            default: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        Cast {
                            id: _,
                            position,
                            input: _,
                            name: _,
                            column: _,
                            data_type: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        Compute {
                            id: _,
                            position,
                            input: _,
                            name: _,
                            column: _,
                            operation: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        Filter {
                            id: _,
                            position,
                            input: _,
                            column: _,
                            predicate: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        Join {
                            id: _,
                            position,
                            left_input: _,
                            right_input: _,
                            join_type: _,
                            columns: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        LoadCsv {
                            id: _,
                            position,
                            filename: _,
                            outputs: _
                        } => {
                            let undo = Operation::SetPosition {
                                id,
                                position: position.clone()
                            };
                            *position = new_position;
                            Ok(Some(undo))
                        }

                        Select {
                            id: _,
                            position,
                            input: _,
                            columns: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        Sort {
                            id: _,
                            position,
                            input: _,
                            sorters: _,
                            outputs: _
                        } => set_position!(id, position, new_position),

                        Union {
                            id: _,
                            position,
                            primary_input: _,
                            secondary_input: _,
                            outputs: _
                        } => set_position!(id, position, new_position),
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            InsertAggregate { id, index, aggregate } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Aggregate {
                        id: _,
                        position: _,
                        input: _,
                        aggregates,
                        outputs: _
                    } = node {
                        if index <= aggregates.len() {
                            aggregates.splice(index..index, [aggregate]);
                            let undo = Operation::DeleteAggregate {
                                id,
                                index
                            };
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("Can't insert a new aggregate at index {}. Possible index (0 - {})", index, aggregates.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't insert an aggregate into a non-aggregate node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            DeleteAggregate { id, index } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Aggregate {
                        id: _,
                        position: _,
                        input: _,
                        aggregates,
                        outputs: _
                    } = node {
                        if index < aggregates.len() {
                            let undo = InsertAggregate {
                                id,
                                index,
                                aggregate: aggregates[index].clone()
                            };
                            let end = index + 1;
                            aggregates.splice(index..end, []);
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no aggregate at index {}. Possible index (0 - {})", index, aggregates.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't delete an aggregate from a non-aggregate node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetAggregateComputation { id, index, computation } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Aggregate {
                        id: _,
                        position: _,
                        input: _,
                        aggregates,
                        outputs: _
                    } = node {
                        if index < aggregates.len() {
                            let undo = Operation::SetAggregateComputation {
                                id,
                                index,
                                computation: aggregates[index].computation.clone()
                            };
                            aggregates[index].computation = computation;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no aggregate at index {}. Possible index (0 - {})", index, aggregates.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set aggregate computation to a non-aggregate node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetAggregateColumn { id, index, column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Aggregate {
                        id: _,
                        position: _,
                        input: _,
                        aggregates,
                        outputs: _
                    } = node {
                        if index < aggregates.len() {
                            let undo = Operation::SetAggregateColumn {
                                id,
                                index,
                                column: aggregates[index].column.clone()
                            };
                            aggregates[index].column = column;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no aggregate at index {}. Possible index (0 - {})", index, aggregates.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set aggregate column to a non-aggregate node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetAggregateAlias { id, index, alias } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Aggregate {
                        id: _,
                        position: _,
                        input: _,
                        aggregates,
                        outputs: _
                    } = node {
                        if index < aggregates.len() {
                            let undo = Operation::SetAggregateAlias {
                                id,
                                index,
                                alias: aggregates[index].alias.clone()
                            };
                            aggregates[index].alias = alias;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no aggregate at index {}. Possible index (0 - {})", index, aggregates.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set aggregate alias to a non-aggregate node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            // Bins node operations

            SetBinsName { id, name: new_name } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Bins {
                        id: _,
                        position: _,
                        input: _,
                        name,
                        column: _,
                        lower_bound: _,
                        upper_bound: _,
                        count: _,
                        outputs: _
                    } = node {
                        let undo = SetBinsName { id, name: name.clone() };
                        *name = new_name;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set bins name to a non-bins node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetBinsColumn { id, column: new_column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Bins {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        column,
                        lower_bound: _,
                        upper_bound: _,
                        count: _,
                        outputs: _
                    } = node {
                        let undo = SetBinsColumn { id, column: column.clone() };
                        *column = new_column;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set bins column to a non-bins node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetBinsLowerBound { id, lower_bound: new_lower_bound } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Bins {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        column: _,
                        lower_bound,
                        upper_bound: _,
                        count: _,
                        outputs: _
                    } = node {
                        let undo = SetBinsLowerBound { id, lower_bound: *lower_bound };
                        *lower_bound = new_lower_bound;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set bins lower bound to a non-bins node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetBinsUpperBound { id, upper_bound: new_upper_bound } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Bins {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        column: _,
                        lower_bound: _,
                        upper_bound,
                        count: _,
                        outputs: _
                    } = node {
                        let undo = SetBinsUpperBound { id, upper_bound: *upper_bound };
                        *upper_bound = new_upper_bound;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set bins upper bound to a non-bins node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetBinsCount { id, count: new_count } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Bins {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        column: _,
                        lower_bound: _,
                        upper_bound: _,
                        count,
                        outputs: _
                    } = node {
                        let undo = SetBinsCount { id, count: *count };
                        *count = new_count;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set bins count to a non-bins node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            // Case node operations
            SetCaseName { id, name: new_name } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Case {
                        id: _,
                        position: _,
                        input: _,
                        name,
                        data_type: _,
                        cases: _,
                        default: _,
                        outputs: _
                    } = node {
                        let undo = SetCaseName { id, name: name.clone() };
                        *name = new_name;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set case name to a non-case node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetCaseDataType { id, data_type: new_data_type } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Case {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        data_type,
                        cases: _,
                        default: _,
                        outputs: _
                    } = node {
                        let undo = SetCaseDataType {
                            id,
                            data_type: data_type.clone()
                        };
                        *data_type = new_data_type;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set case data type to a non-case node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            InsertCase { id, index, case } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Case {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        data_type: _,
                        cases,
                        default: _,
                        outputs: _
                    } = node {
                        if index <= cases.len() {
                            cases.splice(index..index, [case]);
                            let undo = Operation::DeleteCase {
                                id,
                                index
                            };
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("Can't insert a new case at index {}. Possible index (0 - {})", index, cases.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't insert a case to a non-case node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            DeleteCase { id, index } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Case {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        data_type: _,
                        cases,
                        default: _,
                        outputs: _
                    } = node {
                        if index < cases.len() {
                            let undo = InsertCase {
                                id,
                                index,
                                case: cases[index].clone()
                            };
                            let end = index + 1;
                            cases.splice(index..end, []);
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("Can't delete case at index {}. Possible index (0 - {})", index, cases.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't delete case on a non-case node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetCaseColumn { id, index, column: new_column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Case {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        data_type: _,
                        cases,
                        default: _,
                        outputs: _
                    } = node {
                        if index < cases.len() {
                            let undo = SetCaseColumn {
                                id,
                                index,
                                column: cases[index].column.clone()
                            };
                            cases[index].column = new_column;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("Can't set case column at index {}. Possible index (0 - {})", index, cases.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set case column on a non-case node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetCaseValue { id, index, value: new_value } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Case {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        data_type: _,
                        cases,
                        default: _,
                        outputs: _
                    } = node {
                        if index < cases.len() {
                            let undo = SetCaseValue {
                                id,
                                index,
                                value: cases[index].value.clone()
                            };
                            cases[index].value = new_value;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("Can't set case value at index {}. Possible index (0 - {})", index, cases.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set case value on a non-case node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetCaseDefault { id, default: new_default } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Case {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        data_type: _,
                        cases: _,
                        default,
                        outputs: _
                    } = node {
                        let undo = SetCaseDefault {
                            id,
                            default: default.clone()
                        };
                        *default = new_default;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set case default to a non-case node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            // Cast node operations
            SetCastName { id, name: new_name } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Cast {
                        id: _,
                        position: _,
                        input: _,
                        name,
                        column: _,
                        data_type: _,
                        outputs: _
                    } = node {
                        let undo = SetCastName {
                            id,
                            name: name.clone()
                        };
                        *name = new_name;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set cast name default to a non-cast node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetCastColumn { id, column: new_column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Cast {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        column,
                        data_type: _,
                        outputs: _
                    } = node {
                        let undo = SetCastColumn {
                            id,
                            column: column.clone()
                        };
                        *column = new_column;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set cast column default to a non-cast node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetCastDataType { id, data_type: new_data_type } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Cast {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        column: _,
                        data_type,
                        outputs: _
                    } = node {
                        let undo = SetCastDataType {
                            id,
                            data_type: data_type.clone()
                        };
                        *data_type = new_data_type;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set cast data type default to a non-cast node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            // Compute node operations
            SetComputeName { id, name: new_name } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Compute {
                        id: _,
                        position: _,
                        input: _,
                        name,
                        column: _,
                        operation: _,
                        outputs: _
                    } = node {
                        let undo = SetComputeName {
                            id,
                            name: name.clone()
                        };
                        *name = new_name;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set compute name to a non-compute node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetComputeColumn { id, column: new_column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Compute {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        column,
                        operation: _,
                        outputs: _
                    } = node {
                        let undo = SetComputeColumn {
                            id,
                            column: column.clone()
                        };
                        *column = new_column;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set compute column to a non-compute node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetComputeOperation { id, operation: new_operation } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Compute {
                        id: _,
                        position: _,
                        input: _,
                        name: _,
                        column: _,
                        operation,
                        outputs: _
                    } = node {
                        let undo = SetComputeOperation {
                            id,
                            operation: operation.clone()
                        };
                        *operation = new_operation;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set compute operation to a non-compute node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetFilterColumn { id, column: new_column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Filter {
                        id: _,
                        position: _,
                        input: _,
                        column,
                        predicate: _,
                        outputs: _
                    } = node {
                        let undo = SetFilterColumn {
                            id,
                            column: column.clone()
                        };
                        *column = new_column;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set filter column to a non-filter node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetFilterPredicate { id, predicate: new_predicate } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Filter {
                        id: _,
                        position: _,
                        input: _,
                        column: _,
                        predicate,
                        outputs: _
                    } = node {
                        let undo = SetFilterPredicate {
                            id,
                            predicate: predicate.clone()
                        };
                        *predicate = new_predicate;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set filter predicate to a non-filter node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetLoadCsvFilename { id, filename: new_path } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::LoadCsv {
                        id: _,
                        position: _,
                        filename,
                        outputs: _
                    } = node {
                        let undo = SetLoadCsvFilename {
                            id,
                            filename: filename.clone()
                        };
                        *filename = new_path;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set csv path to a non-load-csv node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetJoinType { id, join_type: new_join_type } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Join {
                        id: _,
                        position: _,
                        left_input: _,
                        right_input: _,
                        join_type,
                        columns: _,
                        outputs: _
                    } = node {
                        let undo = SetJoinType {
                            id,
                            join_type: join_type.clone()
                        };
                        *join_type = new_join_type;
                        Ok(Some(undo))
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set join type to a non-join node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            InsertJoinColumn { id, index, join_column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Join {
                        id: _,
                        position: _,
                        left_input: _,
                        right_input: _,
                        join_type: _,
                        columns,
                        outputs: _
                    } = node {
                        if index <= columns.len() {
                            columns.splice(index..index, [join_column]);
                            let undo = Operation::DeleteJoinColumn {
                                id,
                                index
                            };
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("Can't insert a new join column at index {}. Possible index (0 - {})", index, columns.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't insert a join column into a non-join node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            DeleteJoinColumn { id, index } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Join {
                        id: _,
                        position: _,
                        left_input: _,
                        right_input: _,
                        join_type: _,
                        columns,
                        outputs: _
                    } = node {
                        if index < columns.len() {
                            let undo = InsertJoinColumn {
                                id,
                                index,
                                join_column: columns[index].clone()
                            };
                            let end = index + 1;
                            columns.splice(index..end, []);
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no join column at index {}. Possible index (0 - {})", index, columns.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't delete a join column from a non-join node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetJoinColumnLeft { id, index, column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Join {
                        id: _,
                        position: _,
                        left_input: _,
                        right_input: _,
                        join_type: _,
                        columns,
                        outputs: _
                    } = node {
                        if index < columns.len() {
                            let undo = SetJoinColumnLeft {
                                id,
                                index,
                                column: columns[index].left.clone()
                            };
                            columns[index].left = column;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no join column at index {}. Possible index (0 - {})", index, columns.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set left join column to a non-join node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetJoinColumnRight { id, index, column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Join {
                        id: _,
                        position: _,
                        left_input: _,
                        right_input: _,
                        join_type: _,
                        columns,
                        outputs: _
                    } = node {
                        if index < columns.len() {
                            let undo = SetJoinColumnRight {
                                id,
                                index,
                                column: columns[index].right.clone()
                            };
                            columns[index].right = column;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no join column at index {}. Possible index (0 - {})", index, columns.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set right join column to a non-join node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            InsertSelect { id, index, column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Select {
                        id: _,
                        position: _,
                        input: _,
                        columns,
                        outputs: _
                    } = node {
                        if index <= columns.len() {
                            columns.splice(index..index, [column]);
                            let undo = Operation::DeleteSelect {
                                id,
                                index
                            };
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("Can't insert a new select column at index {}. Possible index (0 - {})", index, columns.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't insert a select column into a non-select node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            DeleteSelect { id, index } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Select {
                        id: _,
                        position: _,
                        input: _,
                        columns,
                        outputs: _
                    } = node {
                        if index < columns.len() {
                            let undo = InsertSelect {
                                id,
                                index,
                                column: columns[index].clone()
                            };
                            let end = index + 1;
                            columns.splice(index..end, []);
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no select column at index {}. Possible index (0 - {})", index, columns.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't delete select column from a non-join node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetSelectColumn { id, index, column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Select {
                        id: _,
                        position: _,
                        input: _,
                        columns,
                        outputs: _
                    } = node {
                        if index < columns.len() {
                            let undo = SetSelectColumn {
                                id,
                                index,
                                column: columns[index].column.clone()
                            };
                            columns[index].column = column;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no select column at index {}. Possible index (0 - {})", index, columns.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set select column to a non-select node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetSelectAlias { id, index, alias } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Select {
                        id: _,
                        position: _,
                        input: _,
                        columns,
                        outputs: _
                    } = node {
                        if index < columns.len() {
                            let undo = SetSelectAlias {
                                id,
                                index,
                                alias: columns[index].alias.clone()
                            };
                            columns[index].alias = alias;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no select alias at index {}. Possible index (0 - {})", index, columns.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set select alias to a non-select node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            InsertSorter { id, index, sorter } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Sort {
                        id: _,
                        position: _,
                        input: _,
                        sorters,
                        outputs: _
                    } = node {
                        if index <= sorters.len() {
                            sorters.splice(index..index, [sorter]);
                            let undo = Operation::DeleteSorter {
                                id,
                                index
                            };
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("Can't insert a new sorter at index {}. Possible index (0 - {})", index, sorters.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't insert a sorter into a non-sort node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            DeleteSorter { id, index } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Sort {
                        id: _,
                        position: _,
                        input: _,
                        sorters,
                        outputs: _
                    } = node {
                        if index < sorters.len() {
                            let undo = InsertSorter {
                                id,
                                index,
                                sorter: sorters[index].clone()
                            };
                            let end = index + 1;
                            sorters.splice(index..end, []);
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no sorter at index {}. Possible index (0 - {})", index, sorters.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't delete sorter from a non-sort node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetSortColumn { id, index, column } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Sort {
                        id: _,
                        position: _,
                        input: _,
                        sorters,
                        outputs: _
                    } = node {
                        if index < sorters.len() {
                            let undo = SetSortColumn {
                                id,
                                index,
                                column: sorters[index].column.clone()
                            };
                            sorters[index].column = column;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no sorter at index {}. Possible index (0 - {})", index, sorters.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set sort column to a non-sort node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
                }
            }

            SetSortDirection { id, index, direction } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    if let Node::Sort {
                        id: _,
                        position: _,
                        input: _,
                        sorters,
                        outputs: _
                    } = node {
                        if index < sorters.len() {
                            let undo = SetSortDirection {
                                id,
                                index,
                                direction: sorters[index].direction.clone()
                            };
                            sorters[index].direction = direction;
                            Ok(Some(undo))
                        } else {
                            Err(PoldaError::OperationError(format!("There's no sorter at index {}. Possible index (0 - {})", index, sorters.len())))
                        }
                    } else {
                        Err(PoldaError::OperationError(format!("Can't set sort direction to a non-sort node")))
                    }
                } else {
                    Err(PoldaError::OperationError(format!("Node with id {} doesn't exist", id)))
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

pub fn collect(
    nodes: &HashMap<String, Node>,
    id: &String,
    limit: Option<usize>
) -> Result<DataFrame, PoldaError> {
    let mut queries: HashMap<String, Query> = HashMap::new();
    let mut polars_queries: HashMap<String, Query> = HashMap::new();
    let mut nodes_to_query = vec![id.clone()];

    while let Some(id) = nodes_to_query.last().cloned() {
        if let Some(node) = nodes.get(&id) {
            let inputs = node.inputs();
            let connected_inputs = inputs
                .iter()
                .filter(|i| i.is_some())
                .map(|i| i.as_ref().unwrap())
                .collect::<Vec<_>>();

            // Ensure the node has all the necessary inputs.
            let dif = inputs.len() - connected_inputs.len();
            if dif == 1 {
                return Err(PoldaError::QueryError(format!("Node {} is missing an input", id)));
            } else if dif > 1 {
                return Err(PoldaError::QueryError(format!("Node {} is missing {} inputs", id, dif)));
            }

            // Ensure all inputs are in queries, otherwise push to
            // nodes_to_query for now and build the query later.
            let mut are_inputs_in_queries = true;
            for input in connected_inputs.iter() {
                if !queries.contains_key(*input) {
                    are_inputs_in_queries = false;
                    nodes_to_query.push((*input).clone());
                }
            }

            if !are_inputs_in_queries {
                continue;
            }

            // Check if the input queries have the same backend.
            let mut same_backend = true;
            if let Some(first_input) = connected_inputs.first() {
                if let Some(first_input_query) = queries.get(*first_input) {
                    for input in connected_inputs[1..].iter() {
                        let input_query = queries.get(*input).unwrap();
                        if !first_input_query.same_backend(input_query) {
                            same_backend = false;
                            break;
                        }
                    }
                }
            }

            let input_queries: Vec<Query> = if !same_backend {
                // Input queries have different backends: Convert them into
                // polars queries.
                for input in connected_inputs.iter() {
                    let input = *input;
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

                connected_inputs
                    .iter()
                    .map(|input| {
                        polars_queries.get(*input).unwrap().clone()
                    })
                    .collect()
            } else {
                // Input queries have the same backend: Build the query.
                connected_inputs
                    .iter()
                    .map(|input| {
                        queries.get(*input).unwrap().clone()
                    })
                    .collect()
            };

            let query = Query::from_node(node, input_queries)?;
            queries.insert(id.clone(), query);
            nodes_to_query.pop();
        } else {
            // Doc is broken: There's a node that contains an input that
            // doesn't exist.
            return Err(PoldaError::DocError(format!("Node with id \"{}\" doesn't exist", id)));
        }
    }

    let df = queries.remove(id).unwrap().collect()?;

    Ok(df.head(limit))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn execute_operations() {
        let mut doc = Doc::new();
        let ops = vec![
            Operation::InsertNode {
                node: Node::LoadCsv {
                    id: "a".to_string(),
                    position: Position {
                        x: 0.0,
                        y: 0.0
                    },
                    filename: "data/supermarket_sales.csv".to_string(),
                    outputs: HashSet::new()
                }
            },
            Operation::InsertIndex {
                id: "a".to_string(),
                index: 0
            },
            Operation::InsertNode {
                node: Node::Select {
                    id: "b".to_string(),
                    position: Position {
                        x: 0.0,
                        y: 0.0
                    },
                    input: None,
                    columns: vec![
                        SelectColumn {
                            column: "City".to_string(),
                            alias: "".to_string()
                        },
                        SelectColumn {
                            column: "Product line".to_string(),
                            alias: "".to_string()
                        }
                    ],
                    outputs: HashSet::new()
                }
            },
            Operation::InsertIndex {
                id: "b".to_string(),
                index: 0
            },
            Operation::SetInput {
                id: "b".to_string(),
                name: InputName::Primary,
                input: Some("a".to_string())
            },
            Operation::InsertNode {
                node: Node::Aggregate {
                    id: "c".to_string(),
                    position: Position {
                        x: 0.0,
                        y: 0.0
                    },
                    input: None,
                    aggregates: vec![
                        Aggregate {
                            column: String::from("City"),
                            computation: AggregateComputation::Group,
                            alias: String::from("")
                        },
                        Aggregate {
                            column: String::from("Product line"),
                            computation: AggregateComputation::Group,
                            alias: String::from("")
                        },
                        Aggregate {
                            column: String::from("Quantity"),
                            computation: AggregateComputation::Sum,
                            alias: String::from("")
                        },
                        Aggregate {
                            column: String::from("Total"),
                            computation: AggregateComputation::Sum,
                            alias: String::from("")
                        },
                        Aggregate {
                            column: String::from("Total"),
                            computation: AggregateComputation::Mean,
                            alias: String::from("Mean")
                        }
                    ],
                    outputs: HashSet::new()
                }
            },
            Operation::InsertIndex {
                id: "c".to_string(),
                index: 0
            },
            Operation::SetInput {
                id: "c".to_string(),
                name: InputName::Primary,
                input: Some("a".to_string())
            },
            Operation::InsertNode {
                node: Node::Sort {
                    id: "d".to_string(),
                    position: Position {
                        x: 0.0,
                        y: 0.0
                    },
                    input: None,
                    sorters: vec![
                        Sorter {
                            column: String::from("Total"),
                            direction: SortDirection::Desc
                        }
                    ],
                    outputs: HashSet::new()
                }
            },
            Operation::InsertIndex {
                id: "d".to_string(),
                index: 0
            },
            Operation::SetInput {
                id: "d".to_string(),
                name: InputName::Primary,
                input: Some("c".to_string())
            }
        ];
        doc.execute_operations(ops).unwrap();
        println!("{:#?}", doc);
        let df = doc.collect(&String::from("d"), None).unwrap();
        println!("{:#?}", df);
    }
}
