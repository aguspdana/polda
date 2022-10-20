use crate::node::Position;
use super::Filter;
use super::FilterPredicate;

#[derive(Debug, Clone)]
pub enum FilterNodeOperation {
    SetPosition {
        position: Position
    },
    SetInput {
        input: Option<String>
    },
    InsertFilter {
        index: usize,
        filter: Filter
    },
    DeleteFilter {
        index: usize
    },
    SetColumn {
        index: usize,
        column: String
    },
    SetPredicate {
        index: usize,
        predicate: FilterPredicate
    }
}

impl FilterNodeOperation {
    pub fn map (self, mapper: &FilterNodeOperation) -> FilterNodeOperation {
        match (mapper, self) {
            (
                FilterNodeOperation::InsertFilter { index: map_index, filter: _ },
                FilterNodeOperation::InsertFilter { index: _, filter }
            ) => FilterNodeOperation::InsertFilter { index: *map_index, filter },

            (
                FilterNodeOperation::InsertFilter { index: map_index, filter: _ },
                FilterNodeOperation::DeleteFilter { index: _ }
            ) => FilterNodeOperation::DeleteFilter { index: *map_index },

            (
                FilterNodeOperation::InsertFilter { index: map_index, filter: _ },
                FilterNodeOperation::SetColumn { index: _, column }
            ) => FilterNodeOperation::SetColumn { index: *map_index, column },

            (
                FilterNodeOperation::InsertFilter { index: map_index, filter: _ },
                FilterNodeOperation::SetPredicate { index: _, predicate }
            ) => FilterNodeOperation::SetPredicate { index: *map_index, predicate },

            (map, op) => panic!("Can't map {:?} to {:?}", op, map)
        }
    }

    pub fn transform_backward(self, preceded_by: &FilterNodeOperation) -> Option<FilterNodeOperation> {
        match (preceded_by, self) {
            (
                FilterNodeOperation::InsertFilter { index: pre_index, filter: _ },
                FilterNodeOperation::InsertFilter { mut index, filter }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(FilterNodeOperation::InsertFilter { index, filter })
                }
            }

            (
                FilterNodeOperation::InsertFilter { index: pre_index, filter: _ },
                FilterNodeOperation::DeleteFilter { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(FilterNodeOperation::DeleteFilter { index })
                }
            }

            (
                FilterNodeOperation::InsertFilter { index: pre_index, filter: _ },
                FilterNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(FilterNodeOperation::SetColumn { index, column })
                }
            }

            (
                FilterNodeOperation::InsertFilter { index: pre_index, filter: _ },
                FilterNodeOperation::SetPredicate { mut index, predicate }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(FilterNodeOperation::SetPredicate { index, predicate })
                }
            }

            (
                FilterNodeOperation::DeleteFilter { index: pre_index },
                FilterNodeOperation::InsertFilter { mut index, filter }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(FilterNodeOperation::InsertFilter { index, filter })
            }

            (
                FilterNodeOperation::DeleteFilter { index: pre_index },
                FilterNodeOperation::DeleteFilter { mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(FilterNodeOperation::DeleteFilter { index })
            }

            (
                FilterNodeOperation::DeleteFilter { index: pre_index },
                FilterNodeOperation::SetColumn { mut index, column }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(FilterNodeOperation::SetColumn { index, column })
            }

            (
                FilterNodeOperation::DeleteFilter { index: pre_index },
                FilterNodeOperation::SetPredicate { mut index, predicate }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(FilterNodeOperation::SetPredicate { index, predicate })
            }

            (_, op) => Some(op)
        }
    }

    pub fn transform_forward(self, preceded_by: &FilterNodeOperation) -> Option<FilterNodeOperation> {
        match (preceded_by, self) {
            (
                FilterNodeOperation::InsertFilter { index: pre_index, filter: _ },
                FilterNodeOperation::InsertFilter { mut index, filter }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index += 1;
                    }
                    Some(FilterNodeOperation::InsertFilter { index, filter })
                }
            }

            (
                FilterNodeOperation::InsertFilter { index: pre_index, filter: _ },
                FilterNodeOperation::DeleteFilter { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index += 1;
                    }
                    Some(FilterNodeOperation::DeleteFilter { index })
                }
            }

            (
                FilterNodeOperation::InsertFilter { index: pre_index, filter: _ },
                FilterNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index += 1;
                    }
                    Some(FilterNodeOperation::SetColumn { index, column })
                }
            }

            (
                FilterNodeOperation::InsertFilter { index: pre_index, filter: _ },
                FilterNodeOperation::SetPredicate { mut index, predicate }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index += 1;
                    }
                    Some(FilterNodeOperation::SetPredicate { index, predicate })
                }
            }

            (
                FilterNodeOperation::DeleteFilter { index: pre_index },
                FilterNodeOperation::InsertFilter { mut index, filter }
            ) => {
                if index > *pre_index {
                    index -= 1;
                }
                Some(FilterNodeOperation::InsertFilter { index, filter })
            }

            (
                FilterNodeOperation::DeleteFilter { index: pre_index },
                FilterNodeOperation::DeleteFilter { mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(FilterNodeOperation::DeleteFilter { index })
                }
            }

            (
                FilterNodeOperation::DeleteFilter { index: pre_index },
                FilterNodeOperation::SetColumn { mut index, column }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(FilterNodeOperation::SetColumn { index, column })
                }
            }

            (
                FilterNodeOperation::DeleteFilter { index: pre_index },
                FilterNodeOperation::SetPredicate { mut index, predicate }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(FilterNodeOperation::SetPredicate { index, predicate })
                }
            }

            (_, op) => Some(op)
        }
    }
}
