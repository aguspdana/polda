use crate::node::Position;

use super::Aggregate;
use super::AggregateComputation;

#[derive(Debug, Clone)]
pub enum AggregateNodeOperation {
    SetPosition {
        position: Position
    },
    SetInput {
        input: Option<String>
    },
    InsertAggregate {
        index: usize,
        aggregate: Aggregate
    },
    DeleteAggregate {
        index: usize
    },
    SetComputation {
        index: usize,
        computation: AggregateComputation
    },
    SetColumn {
        index: usize,
        column: String
    },
    SetAlias {
        index: usize,
        alias: String
    },
}

impl AggregateNodeOperation {
    pub fn map(self, mapper: &AggregateNodeOperation) -> AggregateNodeOperation {
        match (mapper, self) {
            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::DeleteAggregate { index: _ }
            ) => {
                AggregateNodeOperation::DeleteAggregate { index: *pre_index }
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetComputation { index: _, computation }
            ) => {
                AggregateNodeOperation::SetComputation { index: *pre_index, computation }
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetColumn { index: _, column }
            ) => {
                AggregateNodeOperation::SetColumn { index: *pre_index, column }
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetAlias { index: _, alias }
            ) => {
                AggregateNodeOperation::SetAlias { index: *pre_index, alias }
            }

            (a, b) => panic!("Can't map {:?} to {:?}", b, a)
        }
    }

    pub fn transform_backward(self, preceded_by: &AggregateNodeOperation) -> Option<AggregateNodeOperation> {
        match (preceded_by, self) {
            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::InsertAggregate { mut index, aggregate }
            ) => {
                if index > *pre_index {
                    index -= 1;
                }
                Some(AggregateNodeOperation::InsertAggregate { index, aggregate })
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::DeleteAggregate { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(AggregateNodeOperation::DeleteAggregate { index })
                }
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetComputation { mut index, computation }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(AggregateNodeOperation::SetComputation { index, computation })
                }
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(AggregateNodeOperation::SetColumn { index, column })
                }
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetAlias { mut index, alias }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(AggregateNodeOperation::SetAlias { index, alias })
                }
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::InsertAggregate { mut index, aggregate }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::InsertAggregate { index, aggregate })
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::DeleteAggregate { mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::DeleteAggregate { index })
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::SetComputation { mut index, computation }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::SetComputation { index, computation })
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::SetColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::SetColumn { index, column })
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::SetAlias { mut index, alias }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::SetAlias { index, alias })
            }

            (_, op) => Some(op)
        }
    }

    pub fn transform_forward(self, preceded_by: &AggregateNodeOperation) -> Option<AggregateNodeOperation> {
        match (preceded_by, self) {
            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _},
                AggregateNodeOperation::InsertAggregate { mut index, aggregate }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::InsertAggregate { index, aggregate })
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::DeleteAggregate { mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::DeleteAggregate { index })
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetComputation { mut index, computation }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::SetComputation { index, computation })
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::SetColumn { index, column })
            }

            (
                AggregateNodeOperation::InsertAggregate { index: pre_index, aggregate: _ },
                AggregateNodeOperation::SetAlias { mut index, alias }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(AggregateNodeOperation::SetAlias { index, alias })
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::InsertAggregate { mut index, aggregate }
            ) => {
                if index > *pre_index {
                    index -= 1;
                }
                Some(AggregateNodeOperation::InsertAggregate { index, aggregate })
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::DeleteAggregate { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(AggregateNodeOperation::DeleteAggregate { index })
                }
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::SetComputation { mut index, computation }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(AggregateNodeOperation::SetComputation { index , computation })
                }
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(AggregateNodeOperation::SetColumn { index, column })
                }
            }

            (
                AggregateNodeOperation::DeleteAggregate { index: pre_index },
                AggregateNodeOperation::SetAlias { mut index, alias }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(AggregateNodeOperation::SetAlias { index, alias })
                }
            }

            (_, op) => Some(op)
        }
    }
}
