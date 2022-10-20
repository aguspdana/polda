use crate::node::Position;

use super::JoinType;
use super::JoinColumn;

#[derive(Debug, Clone)]
pub enum JoinNodeOperation {
    SetPosition {
        position: Position
    },
    SetLeftInput {
        input: Option<String>
    },
    SetRightInput {
        input: Option<String>
    },
    SetJoinType {
        join_type: JoinType
    },
    InsertJoinColumn {
        index: usize,
        column: JoinColumn
    },
    DeleteJoinColumn {
        index: usize
    },
    SetLeftColumn {
        index: usize,
        column: String
    },
    SetRightColumn {
        index: usize,
        column: String
    }
}

impl JoinNodeOperation {
    pub fn map(self, mapper: &JoinNodeOperation) -> JoinNodeOperation {
        match (mapper, self) {
            (
                JoinNodeOperation::InsertJoinColumn { index: map_index, column: _ },
                JoinNodeOperation::InsertJoinColumn { index: _, column }
            ) => JoinNodeOperation::InsertJoinColumn { index: *map_index, column },

            (
                JoinNodeOperation::InsertJoinColumn { index: map_index, column: _ },
                JoinNodeOperation::DeleteJoinColumn { index: _ }
            ) => JoinNodeOperation::DeleteJoinColumn { index: *map_index },

            (
                JoinNodeOperation::InsertJoinColumn { index: map_index, column: _ },
                JoinNodeOperation::SetLeftColumn { index: _, column }
            ) => JoinNodeOperation::SetLeftColumn { index: *map_index, column },

            (
                JoinNodeOperation::InsertJoinColumn { index: map_index, column: _ },
                JoinNodeOperation::SetRightColumn { index: _, column }
            ) => JoinNodeOperation::SetRightColumn { index: *map_index, column },

            (a, b) => panic!("Can't map {:?} to {:?}", b, a)
        }
    }

    pub fn transform_backward(self, preceded_by: &JoinNodeOperation) -> Option<JoinNodeOperation> {
        match (preceded_by, self) {
            (
                JoinNodeOperation::InsertJoinColumn { index: pre_index, column: _ },
                JoinNodeOperation::InsertJoinColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(JoinNodeOperation::InsertJoinColumn { index, column })
                }
            }

            (
                JoinNodeOperation::InsertJoinColumn { index: pre_index, column: _ },
                JoinNodeOperation::DeleteJoinColumn { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(JoinNodeOperation::DeleteJoinColumn { index })
                }
            }

            (
                JoinNodeOperation::InsertJoinColumn { index: pre_index, column: _ },
                JoinNodeOperation::SetLeftColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(JoinNodeOperation::SetLeftColumn { index, column })
                }
            }

            (
                JoinNodeOperation::InsertJoinColumn { index: pre_index, column: _ },
                JoinNodeOperation::SetRightColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(JoinNodeOperation::SetRightColumn { index, column })
                }
            }

            (
                JoinNodeOperation::DeleteJoinColumn { index: pre_index },
                JoinNodeOperation::InsertJoinColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(JoinNodeOperation::InsertJoinColumn { index, column })
            }

            (
                JoinNodeOperation::DeleteJoinColumn { index: pre_index },
                JoinNodeOperation::DeleteJoinColumn { mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(JoinNodeOperation::DeleteJoinColumn { index })
            }

            (
                JoinNodeOperation::DeleteJoinColumn { index: pre_index },
                JoinNodeOperation::SetLeftColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(JoinNodeOperation::SetLeftColumn { index, column })
            }

            (
                JoinNodeOperation::DeleteJoinColumn { index: pre_index },
                JoinNodeOperation::SetRightColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(JoinNodeOperation::SetRightColumn { index, column })
            }

            (_, op) => Some(op)
        }
    }

    pub fn transform_forward(self, preceded_by: &JoinNodeOperation) -> Option<JoinNodeOperation> {
        match (preceded_by, self) {
            (
                JoinNodeOperation::InsertJoinColumn { index: pre_index, column: _ },
                JoinNodeOperation::InsertJoinColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(JoinNodeOperation::InsertJoinColumn { index, column })
            }

            (
                JoinNodeOperation::InsertJoinColumn { index: pre_index, column: _ },
                JoinNodeOperation::DeleteJoinColumn { mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(JoinNodeOperation::DeleteJoinColumn { index })
            }

            (
                JoinNodeOperation::InsertJoinColumn { index: pre_index, column: _ },
                JoinNodeOperation::SetLeftColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(JoinNodeOperation::SetLeftColumn { index, column })
            }

            (
                JoinNodeOperation::InsertJoinColumn { index: pre_index, column: _ },
                JoinNodeOperation::SetRightColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(JoinNodeOperation::SetRightColumn { index, column })
            }

            (
                JoinNodeOperation::DeleteJoinColumn { index: pre_index },
                JoinNodeOperation::InsertJoinColumn { mut index, column }
            ) => {
                if index > *pre_index {
                    index -= 1;
                }
                Some(JoinNodeOperation::InsertJoinColumn { index, column })
            }

            (
                JoinNodeOperation::DeleteJoinColumn { index: pre_index },
                JoinNodeOperation::DeleteJoinColumn { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(JoinNodeOperation::DeleteJoinColumn { index })
                }
            }

            (
                JoinNodeOperation::DeleteJoinColumn { index: pre_index },
                JoinNodeOperation::SetLeftColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(JoinNodeOperation::SetLeftColumn { index, column })
                }
            }

            (
                JoinNodeOperation::DeleteJoinColumn { index: pre_index },
                JoinNodeOperation::SetRightColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(JoinNodeOperation::SetRightColumn { index, column })
                }
            }

            (_, op) => Some(op)
        }
    }
}
