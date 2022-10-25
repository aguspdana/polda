use crate::error::PoldaError;

pub mod aggregate;
pub mod filter;
pub mod join;
pub mod load_csv;
pub mod operation;
pub mod position;
pub mod select;
pub mod sort;
pub mod union;
pub mod value;

pub use aggregate::AggregateNode;
pub use filter::FilterNode;
pub use join::JoinNode;
pub use load_csv::LoadCsvNode;
pub use operation::NodeOperation;
pub use position::Position;
pub use select::SelectNode;
pub use sort::SortNode;
pub use union::UnionNode;
pub use value::Value;

#[derive(Debug, Clone)]
pub enum Node {
    Aggregate(AggregateNode),
    Filter(FilterNode),
    Join(JoinNode),
    LoadCsv(LoadCsvNode),
    Select(SelectNode),
    Sort(SortNode),
    Union(UnionNode)
}

impl Node {
    pub fn inputs(&self) -> Vec<String> {
        match self {
            Node::Aggregate(node) => node.inputs(),
            Node::Filter(node) => node.inputs(),
            Node::Join(node) => node.inputs(),
            Node::LoadCsv(node) => node.inputs(),
            Node::Select(node) => node.inputs(),
            Node::Sort(node) => node.inputs(),
            Node::Union(node) => node.inputs()
        }
    }

    pub fn outputs(&self) -> Vec<String> {
        match self {
            Node::Aggregate(node) => node.outputs(),
            Node::Filter(node) => node.outputs(),
            Node::Join(node) => node.outputs(),
            Node::LoadCsv(node) => node.outputs(),
            Node::Select(node) => node.outputs(),
            Node::Sort(node) => node.outputs(),
            Node::Union(node) => node.outputs()
        }
    }

    pub fn insert_output(&mut self, to: String) {
        match self {
            Node::Aggregate(node) => node.insert_output(to),
            Node::Filter(node) => node.insert_output(to),
            Node::Join(node) => node.insert_output(to),
            Node::LoadCsv(node) => node.insert_output(to),
            Node::Select(node) => node.insert_output(to),
            Node::Sort(node) => node.insert_output(to),
            Node::Union(node) => node.insert_output(to)
        }
    }

    pub fn remove_output(&mut self, to: &String) {
        match self {
            Node::Aggregate(node) => node.remove_output(to),
            Node::Filter(node) => node.remove_output(to),
            Node::Join(node) => node.remove_output(to),
            Node::LoadCsv(node) => node.remove_output(to),
            Node::Select(node) => node.remove_output(to),
            Node::Sort(node) => node.remove_output(to),
            Node::Union(node) => node.remove_output(to)
        }
    }

    /// Create operations that can be used to reset inputs with the given id.
    pub fn reset_input_operations(&self, id: &String) -> Vec<NodeOperation> {
        match self {
            Node::Aggregate(node) => {
                node.reset_input_operations(id)
                    .into_iter()
                    .map(|op| NodeOperation::Aggregate(op))
                    .collect()
            }
            Node::Filter(node) => {
                node.reset_input_operations(id)
                    .into_iter()
                    .map(|op| NodeOperation::Filter(op))
                    .collect()
            }
            Node::Join(node) => {
                node.reset_input_operations(id)
                    .into_iter()
                    .map(|op| NodeOperation::Join(op))
                    .collect()
            }
            Node::LoadCsv(node) => {
                node.reset_input_operations(id)
                    .into_iter()
                    .map(|op| NodeOperation::LoadCsv(op))
                    .collect()
            }
            Node::Select(node) => {
                node.reset_input_operations(id)
                    .into_iter()
                    .map(|op| NodeOperation::Select(op))
                    .collect()
            }
            Node::Sort(node) => {
                node.reset_input_operations(id)
                    .into_iter()
                    .map(|op| NodeOperation::Sort(op))
                    .collect()
            }
            Node::Union(node) => {
                node.reset_input_operations(id)
                    .into_iter()
                    .map(|op| NodeOperation::Union(op))
                    .collect()
            }
        }
    }

    pub fn execute_operation(&mut self, operation: NodeOperation) -> Result<NodeOperation, PoldaError> {
        match (self, operation) {
            (Node::Aggregate(node), NodeOperation::Aggregate(op)) => {
                node.execute_operation(op)
                    .map(|undo| NodeOperation::Aggregate(undo))
            }

            (Node::Filter(node), NodeOperation::Filter(op)) => {
                node.execute_operation(op)
                    .map(|undo| NodeOperation::Filter(undo))
            }

            (Node::Join(node), NodeOperation::Join(op)) => {
                node.execute_operation(op)
                    .map(|undo| NodeOperation::Join(undo))
            }

            (Node::LoadCsv(node), NodeOperation::LoadCsv(op)) => {
                node.execute_operation(op)
                    .map(|undo| NodeOperation::LoadCsv(undo))
            }

            (Node::Select(node), NodeOperation::Select(op)) => {
                node.execute_operation(op)
                    .map(|undo| NodeOperation::Select(undo))
            }

            (Node::Sort(node), NodeOperation::Sort(op)) => {
                node.execute_operation(op)
                    .map(|undo| NodeOperation::Sort(undo))
            }

            (Node::Union(node), NodeOperation::Union(op)) => {
                node.execute_operation(op)
                    .map(|undo| NodeOperation::Union(undo))
            }

            _ => Err(PoldaError::Unsyncable)
        }
    }
}
