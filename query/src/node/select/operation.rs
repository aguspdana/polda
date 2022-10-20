use crate::node::Position;

use super::SelectColumn;

#[derive(Debug, Clone)]
pub enum SelectNodeOperation {
    SetPosition {
        position: Position
    },
    SetInput {
        input: Option<String>
    },
    InsertColumn {
        index: usize,
        column: SelectColumn
    },
    DeleteColumn {
        index: usize
    },
    SetColumn {
        index: usize,
        column: String
    },
    SetAlias {
        index: usize,
        alias: String
    }
}

impl SelectNodeOperation {
    pub fn map(self, mapper: &SelectNodeOperation) -> SelectNodeOperation {
        match (mapper, self) {
            (
                SelectNodeOperation::InsertColumn { index: map_index, column: _ },
                SelectNodeOperation::InsertColumn { index: _, column }
            ) => SelectNodeOperation::InsertColumn { index: *map_index, column },

            (
                SelectNodeOperation::InsertColumn { index: map_index, column: _ },
                SelectNodeOperation::DeleteColumn { index: _ }
            ) => SelectNodeOperation::DeleteColumn { index: *map_index },

            (
                SelectNodeOperation::InsertColumn { index: map_index, column: _ },
                SelectNodeOperation::SetColumn { index: _, column }
            ) => SelectNodeOperation::SetColumn { index: *map_index, column },

            (
                SelectNodeOperation::InsertColumn { index: map_index, column: _ },
                SelectNodeOperation::SetAlias { index: _, alias }
            ) => SelectNodeOperation::SetAlias { index: *map_index, alias },

            (map, op) => panic!("Can't map {:?} to {:?}", op, map)
        }
    }

    pub fn transform_backward(self, preceded_by: &SelectNodeOperation) -> Option<SelectNodeOperation> {
        match (preceded_by, self) {
            (
                SelectNodeOperation::InsertColumn { index: pre_index, column: _ },
                SelectNodeOperation::InsertColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SelectNodeOperation::InsertColumn { index, column })
                }
            }

            (
                SelectNodeOperation::InsertColumn { index: pre_index, column: _ },
                SelectNodeOperation::DeleteColumn { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SelectNodeOperation::DeleteColumn { index })
                }
            }

            (
                SelectNodeOperation::InsertColumn { index: pre_index, column: _ },
                SelectNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SelectNodeOperation::SetColumn { index, column })
                }
            }

            (
                SelectNodeOperation::InsertColumn { index: pre_index, column: _ },
                SelectNodeOperation::SetAlias { mut index, alias }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SelectNodeOperation::SetAlias { index, alias })
                }
            }

            (
                SelectNodeOperation::DeleteColumn { index: pre_index },
                SelectNodeOperation::InsertColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SelectNodeOperation::InsertColumn { index, column })
            }

            (
                SelectNodeOperation::DeleteColumn { index: pre_index },
                SelectNodeOperation::DeleteColumn { mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SelectNodeOperation::DeleteColumn { index })
            }

            (
                SelectNodeOperation::DeleteColumn { index: pre_index },
                SelectNodeOperation::SetColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SelectNodeOperation::SetColumn { index, column })
            }

            (
                SelectNodeOperation::DeleteColumn { index: pre_index },
                SelectNodeOperation::SetAlias { mut index, alias }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SelectNodeOperation::SetAlias { index, alias })
            }

            (_, op) => Some(op)
        }
    }

    pub fn transform_forward(self, preceded_by: &SelectNodeOperation) -> Option<SelectNodeOperation> {
        match (preceded_by, self) {
            (
                SelectNodeOperation::InsertColumn { index: pre_index, column: _ },
                SelectNodeOperation::InsertColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SelectNodeOperation::InsertColumn { index, column })
            }

            (
                SelectNodeOperation::InsertColumn { index: pre_index, column: _ },
                SelectNodeOperation::DeleteColumn { mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SelectNodeOperation::DeleteColumn { index })
            }

            (
                SelectNodeOperation::InsertColumn { index: pre_index, column: _ },
                SelectNodeOperation::SetColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SelectNodeOperation::SetColumn { index, column })
            }

            (
                SelectNodeOperation::InsertColumn { index: pre_index, column: _ },
                SelectNodeOperation::SetAlias { mut index, alias }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SelectNodeOperation::SetAlias { index, alias })
            }

            (
                SelectNodeOperation::DeleteColumn { index: pre_index },
                SelectNodeOperation::InsertColumn { mut index, column }
            ) => {
                if index > *pre_index {
                    index -= 1;
                }
                Some(SelectNodeOperation::InsertColumn { index, column })
            }

            (
                SelectNodeOperation::DeleteColumn { index: pre_index },
                SelectNodeOperation::DeleteColumn { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SelectNodeOperation::DeleteColumn { index })
                }
            }

            (
                SelectNodeOperation::DeleteColumn { index: pre_index },
                SelectNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SelectNodeOperation::SetColumn { index, column })
                }
            }

            (
                SelectNodeOperation::DeleteColumn { index: pre_index },
                SelectNodeOperation::SetAlias { mut index, alias }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SelectNodeOperation::SetAlias { index, alias })
                }
            }

            (_, op) => Some(op)
        }
    }
}
