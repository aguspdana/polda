use crate::node::Position;
use super::Sorter;
use super::SortDirection;

#[derive(Debug, Clone)]
pub enum SortNodeOperation {
    SetPosition {
        position: Position
    },
    SetInput {
        input: Option<String>
    },
    InsertSorter {
        index: usize,
        sorter: Sorter
    },
    DeleteSorter {
        index: usize
    },
    SetColumn {
        index: usize,
        column: String
    },
    SetDirection {
        index: usize,
        direction: SortDirection
    }
}

impl SortNodeOperation {
    pub fn map(self, mapper: &SortNodeOperation) -> SortNodeOperation {
        match (mapper, self) {
            (
                SortNodeOperation::InsertSorter { index: map_index, sorter: _ },
                SortNodeOperation::InsertSorter { index: _, sorter }
            ) => SortNodeOperation::InsertSorter { index: *map_index, sorter },

            (
                SortNodeOperation::InsertSorter { index: map_index, sorter: _ },
                SortNodeOperation::DeleteSorter { index: _ }
            ) => SortNodeOperation::DeleteSorter { index: *map_index },

            (
                SortNodeOperation::InsertSorter { index: map_index, sorter: _ },
                SortNodeOperation::SetColumn { index: _, column }
            ) => SortNodeOperation::SetColumn { index: *map_index, column },

            (
                SortNodeOperation::InsertSorter { index: map_index, sorter: _ },
                SortNodeOperation::SetDirection { index: _, direction }
            ) => SortNodeOperation::SetDirection { index: *map_index, direction },

            (map, op) => panic!("Can't map {:?} to {:?}", op, map)
        }
    }

    pub fn  transform_backward(self, preceded_by: &SortNodeOperation) -> Option<SortNodeOperation> {
        match (preceded_by, self) {
            (
                SortNodeOperation::InsertSorter { index: pre_index, sorter: _ },
                SortNodeOperation::InsertSorter { mut index, sorter }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SortNodeOperation::InsertSorter { index, sorter })
                }
            }

            (
                SortNodeOperation::InsertSorter { index: pre_index, sorter: _ },
                SortNodeOperation::DeleteSorter { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SortNodeOperation::DeleteSorter { index })
                }
            }

            (
                SortNodeOperation::InsertSorter { index: pre_index, sorter: _ },
                SortNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SortNodeOperation::SetColumn { index, column })
                }
            }

            (
                SortNodeOperation::InsertSorter { index: pre_index, sorter: _ },
                SortNodeOperation::SetDirection { mut index, direction }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SortNodeOperation::SetDirection { index, direction })
                }
            }

            (
                SortNodeOperation::DeleteSorter { index: pre_index },
                SortNodeOperation::InsertSorter { mut index, sorter }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SortNodeOperation::InsertSorter { index, sorter })
            }

            (
                SortNodeOperation::DeleteSorter { index: pre_index },
                SortNodeOperation::DeleteSorter { mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SortNodeOperation::DeleteSorter { index })
            }

            (
                SortNodeOperation::DeleteSorter { index: pre_index },
                SortNodeOperation::SetColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SortNodeOperation::SetColumn { index, column })
            }

            (
                SortNodeOperation::DeleteSorter { index: pre_index },
                SortNodeOperation::SetDirection { mut index, direction }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(SortNodeOperation::SetDirection { index, direction })
            }

            (_, op) => Some(op)
        }
    }

    pub fn transform_forward(self, preceded_by: &SortNodeOperation) -> Option<SortNodeOperation> {
        match (preceded_by, self) {
            (
                SortNodeOperation::InsertSorter { index: pre_index, sorter: _ },
                SortNodeOperation::InsertSorter { mut index, sorter }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index += 1;
                    }
                    Some(SortNodeOperation::InsertSorter { index, sorter })
                }
            }

            (
                SortNodeOperation::InsertSorter { index: pre_index, sorter: _ },
                SortNodeOperation::DeleteSorter { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index += 1;
                    }
                    Some(SortNodeOperation::DeleteSorter { index })
                }
            }

            (
                SortNodeOperation::InsertSorter { index: pre_index, sorter: _ },
                SortNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index += 1;
                    }
                    Some(SortNodeOperation::SetColumn { index, column })
                }
            }

            (
                SortNodeOperation::InsertSorter { index: pre_index, sorter: _ },
                SortNodeOperation::SetDirection { mut index, direction }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index += 1;
                    }
                    Some(SortNodeOperation::SetDirection { index, direction })
                }
            }

            (
                SortNodeOperation::DeleteSorter { index: pre_index },
                SortNodeOperation::InsertSorter { mut index, sorter }
            ) => {
                if index > *pre_index {
                    index -= 1;
                }
                Some(SortNodeOperation::InsertSorter { index, sorter })
            }

            (
                SortNodeOperation::DeleteSorter { index: pre_index },
                SortNodeOperation::DeleteSorter { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SortNodeOperation::DeleteSorter { index })
                }
            }

            (
                SortNodeOperation::DeleteSorter { index: pre_index },
                SortNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SortNodeOperation::SetColumn { index, column })
                }
            }

            (
                SortNodeOperation::DeleteSorter { index: pre_index },
                SortNodeOperation::SetDirection { mut index, direction }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(SortNodeOperation::SetDirection { index, direction })
                }
            }

            (_, op) => Some(op)
        }
    }
}
