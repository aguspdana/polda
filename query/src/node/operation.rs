use super::aggregate::AggregateNodeOperation;
use super::filter::FilterNodeOperation;
use super::join::JoinNodeOperation;
use super::load_csv::LoadCsvNodeOperation;
use super::select::SelectNodeOperation;
use super::sort::SortNodeOperation;
use super::union::UnionNodeOperation;

#[derive(Debug, Clone)]
pub enum NodeOperation {
    Aggregate(AggregateNodeOperation),
    Filter(FilterNodeOperation),
    Join(JoinNodeOperation),
    LoadCsv(LoadCsvNodeOperation),
    Select(SelectNodeOperation),
    Sort(SortNodeOperation),
    Union(UnionNodeOperation)
}

impl NodeOperation {
    pub fn map(self, mapper: &NodeOperation) -> NodeOperation {
        match (mapper, self) {
            (NodeOperation::Aggregate(mapper), NodeOperation::Aggregate(op)) => {
                NodeOperation::Aggregate(op.map(mapper))
            }

            (NodeOperation::Filter(mapper), NodeOperation::Filter(op)) => {
                NodeOperation::Filter(op.map(mapper))
            }

            (NodeOperation::Join(mapper), NodeOperation::Join(op)) => {
                NodeOperation::Join(op.map(mapper))
            }

            (NodeOperation::LoadCsv(mapper), NodeOperation::LoadCsv(op)) => {
                NodeOperation::LoadCsv(op.map(mapper))
            }

            (NodeOperation::Select(mapper), NodeOperation::Select(op)) => {
                NodeOperation::Select(op.map(mapper))
            }

            (NodeOperation::Sort(mapper), NodeOperation::Sort(op)) => {
                NodeOperation::Sort(op.map(mapper))
            }

            (NodeOperation::Union(mapper), NodeOperation::Union(op)) => {
                NodeOperation::Union(op.map(mapper))
            }

            (mapper, op) => panic!("Can't map {:?} to {:?}", op, mapper)
        }
    }

    pub fn transform_forward(self, preceded_by: &NodeOperation) -> Option<NodeOperation> {
        match (preceded_by, self) {
            (NodeOperation::Aggregate(pre_op), NodeOperation::Aggregate(op)) => {
                op.transform_forward(pre_op)
                    .map(|op| NodeOperation::Aggregate(op))
            }

            (NodeOperation::Filter(pre_op), NodeOperation::Filter(op)) => {
                op.transform_forward(pre_op)
                    .map(|op| NodeOperation::Filter(op))
            }

            (NodeOperation::Join(pre_op), NodeOperation::Join(op)) => {
                op.transform_forward(pre_op)
                    .map(|op| NodeOperation::Join(op))
            }

            (NodeOperation::LoadCsv(pre_op), NodeOperation::LoadCsv(op)) => {
                op.transform_forward(pre_op)
                    .map(|op| NodeOperation::LoadCsv(op))
            }

            (NodeOperation::Select(pre_op), NodeOperation::Select(op)) => {
                op.transform_forward(pre_op)
                    .map(|op| NodeOperation::Select(op))
            }

            (NodeOperation::Sort(pre_op), NodeOperation::Sort(op)) => {
                op.transform_forward(pre_op)
                    .map(|op| NodeOperation::Sort(op))
            }

            (NodeOperation::Union(pre_op), NodeOperation::Union(op)) => {
                op.transform_forward(pre_op)
                    .map(|op| NodeOperation::Union(op))
            }

            _ => None
        }
    }

    pub fn transform_backward(self, preceded_by: &NodeOperation) -> Option<NodeOperation> {
        match (preceded_by, self) {
            (NodeOperation::Aggregate(pre_op), NodeOperation::Aggregate(op)) => {
                op.transform_backward(pre_op)
                    .map(|op| NodeOperation::Aggregate(op))
            }

            (NodeOperation::Filter(pre_op), NodeOperation::Filter(op)) => {
                op.transform_backward(pre_op)
                    .map(|op| NodeOperation::Filter(op))
            }

            (NodeOperation::Join(pre_op), NodeOperation::Join(op)) => {
                op.transform_backward(pre_op)
                    .map(|op| NodeOperation::Join(op))
            }

            (NodeOperation::LoadCsv(pre_op), NodeOperation::LoadCsv(op)) => {
                op.transform_backward(pre_op)
                    .map(|op| NodeOperation::LoadCsv(op))
            }

            (NodeOperation::Select(pre_op), NodeOperation::Select(op)) => {
                op.transform_backward(pre_op)
                    .map(|op| NodeOperation::Select(op))
            }

            (NodeOperation::Sort(pre_op), NodeOperation::Sort(op)) => {
                op.transform_backward(pre_op)
                    .map(|op| NodeOperation::Sort(op))
            }

            (NodeOperation::Union(pre_op), NodeOperation::Union(op)) => {
                op.transform_backward(pre_op)
                    .map(|op| NodeOperation::Union(op))
            }

            _ => None
        }
    }
}
