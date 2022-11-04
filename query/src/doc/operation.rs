use crate::error::PoldaError;

use super::Node;
use super::InputName;
use super::Position;
use super::Aggregate;
use super::AggregateComputation;
use super::Filter;
use super::FilterPredicate;
use super::JoinColumn;
use super::JoinType;
use super::SelectColumn;
use super::SortDirection;
use super::Sorter;

#[derive(Debug, Clone)]
pub enum Operation {
    // Doc operations:
    InsertNode {
        node: Node
    },
    DeleteNode {
        id: String
    },
    InsertIndex {
        id: String,
        index: usize
    },
    DeleteIndex {
        id: String,
        index: usize
    },

    // Common node operations:
    SetInput {
        id: String,
        name: InputName,
        input: Option<String>
    },
    SetPosition {
        id: String,
        position: Position
    },

    // Aggregate node operations:
    InsertAggregate {
        id: String,
        index: usize,
        aggregate: Aggregate
    },
    DeleteAggregate {
        id: String,
        index: usize
    },
    SetAggregateComputation {
        id: String,
        index: usize,
        computation: AggregateComputation
    },
    SetAggregateColumn {
        id: String,
        index: usize,
        column: String
    },
    SetAggregateAlias {
        id: String,
        index: usize,
        alias: String
    },

    // Filter node operations
    InsertFilter {
        id: String,
        index: usize,
        filter: Filter
    },
    DeleteFilter {
        id: String,
        index: usize
    },
    SetFilterColumn {
        id: String,
        index: usize,
        column: String
    },
    SetFilterPredicate {
        id: String,
        index: usize,
        predicate: FilterPredicate
    },

    // LoadCsv node operations:
    SetCsvPath {
        id: String,
        path: String
    },

    // Join node operations:
    SetJoinType {
        id: String,
        join_type: JoinType
    },
    InsertJoin {
        id: String,
        index: usize,
        join_column: JoinColumn
    },
    DeleteJoin {
        id: String,
        index: usize
    },
    SetLeftJoinColumn {
        id: String,
        index: usize,
        column: String
    },
    SetRightJoinColumn {
        id: String,
        index: usize,
        column: String
    },

    // Select node operations:
    InsertSelect {
        id: String,
        index: usize,
        column: SelectColumn
    },
    DeleteSelect {
        id: String,
        index: usize
    },
    SetSelectColumn {
        id: String,
        index: usize,
        column: String
    },
    SetSelectAlias {
        id: String,
        index: usize,
        alias: String
    },

    // Sort node operations:
    InsertSorter {
        id: String,
        index: usize,
        sorter: Sorter
    },
    DeleteSorter {
        id: String,
        index: usize
    },
    SetSortColumn {
        id: String,
        index: usize,
        column: String
    },
    SetSortDirection {
        id: String,
        index: usize,
        direction: SortDirection
    },

    // Union node operations:
}

impl Operation {
    pub fn id(&self) -> &String {
        use Operation::*;

        match self {
            InsertNode {
                node
            } => node.id(),

            DeleteNode {
                id
            } => id,

            InsertIndex {
                id,
                index: _
            } => id,

            DeleteIndex {
                id,
                index: _
            } => id,

            SetInput {
                id,
                name: _,
                input: _
            } => id,

            SetPosition {
                id,
                position: _
            } => id,

            InsertAggregate {
                id,
                index: _,
                aggregate: _
            } => id,

            DeleteAggregate {
                id,
                index: _
            } => id,

            SetAggregateComputation {
                id,
                index: _,
                computation: _
            } => id,

            SetAggregateColumn {
                id,
                index: _,
                column: _
            } => id,

            SetAggregateAlias {
                id,
                index: _,
                alias: _
            } => id,

            InsertFilter {
                id,
                index: _,
                filter: _
            } => id,

            DeleteFilter {
                id,
                index: _
            } => id,

            SetFilterColumn {
                id,
                index: _,
                column: _
            } => id,

            SetFilterPredicate {
                id,
                index: _,
                predicate: _
            } => id,

            SetCsvPath {
                id,
                path: _
            } => id,

            SetJoinType {
                id,
                join_type: _
            } => id,

            InsertJoin {
                id,
                index: _,
                join_column: _
            } => id,

            DeleteJoin {
                id,
                index: _
            } => id,

            SetLeftJoinColumn {
                id,
                index: _,
                column: _
            } => id,

            SetRightJoinColumn {
                id,
                index: _,
                column: _
            } => id,

            InsertSelect {
                id,
                index: _,
                column: _
            } => id,

            DeleteSelect {
                id,
                index: _
            } => id,

            SetSelectColumn {
                id,
                index: _,
                column: _
            } => id,

            SetSelectAlias {
                id,
                index: _,
                alias: _
            } => id,

            InsertSorter {
                id,
                index: _,
                sorter: _
            } => id,

            DeleteSorter {
                id,
                index: _
            } => id,

            SetSortColumn {
                id,
                index: _,
                column: _
            } => id,

            SetSortDirection {
                id,
                index: _,
                direction: _
            } => id,
        }
    }

    pub fn map(self, mapper: &Operation) -> Operation {
        use Operation::*;

        match (mapper, self) {
            (
                InsertNode { node: _ },
                DeleteNode { id }
            ) => DeleteNode { id },

            (
                InsertNode { node: _ },
                SetInput { id, name, input }
            ) => SetInput { id, name, input },

            (
                InsertNode { node: _ },
                SetPosition { id, position }
            ) => SetPosition { id, position },

            (
                InsertNode { node: _ },
                InsertAggregate { id, index, aggregate }
            ) => InsertAggregate { id, index, aggregate },

            (
                InsertNode { node: _ },
                DeleteAggregate { id, index }
            ) => DeleteAggregate { id, index },

            (
                InsertNode { node: _ },
                SetAggregateComputation { id, index, computation }
            ) => SetAggregateComputation { id, index, computation },

            (
                InsertNode { node: _ },
                SetAggregateColumn { id, index, column }
            ) => SetAggregateColumn { id, index, column },

            (
                InsertNode { node: _ },
                SetAggregateAlias { id, index, alias }
            ) => SetAggregateAlias { id, index, alias },

            (
                InsertNode { node: _ },
                InsertFilter { id, index, filter }
            ) => InsertFilter { id, index, filter },

            (
                InsertNode { node: _ },
                DeleteFilter { id, index }
            ) => DeleteFilter { id, index },

            (
                InsertNode { node: _ },
                SetFilterColumn { id, index, column }
            ) => SetFilterColumn { id, index, column },

            (
                InsertNode { node: _ },
                SetFilterPredicate { id, index, predicate }
            ) => SetFilterPredicate { id, index, predicate },

            (
                InsertNode { node: _ },
                SetCsvPath { id, path }
            ) => SetCsvPath { id, path },

            (
                InsertNode { node: _ },
                SetJoinType { id, join_type }
            ) => SetJoinType { id, join_type },

            (
                InsertNode { node: _ },
                InsertJoin { id, index, join_column }
            ) => InsertJoin { id, index, join_column },

            (
                InsertNode { node: _ },
                DeleteJoin { id, index }
            ) => DeleteJoin { id, index },

            (
                InsertNode { node: _ },
                SetLeftJoinColumn { id, index, column }
            ) => SetLeftJoinColumn { id, index, column },

            (
                InsertNode { node: _ },
                SetRightJoinColumn { id, index, column }
            ) => SetRightJoinColumn { id, index, column },

            (
                InsertNode { node: _ },
                InsertSelect { id, index, column }
            ) => InsertSelect { id, index, column },

            (
                InsertNode { node: _ },
                DeleteSelect { id, index }
            ) => DeleteSelect { id, index },

            (
                InsertNode { node: _ },
                SetSelectColumn { id, index, column }
            ) => SetSelectColumn { id, index, column },

            (
                InsertNode { node: _ },
                SetSelectAlias { id, index, alias }
            ) => SetSelectAlias { id, index, alias },

            (
                InsertNode { node: _ },
                InsertSorter { id, index, sorter }
            ) => InsertSorter { id, index, sorter },

            (
                InsertNode { node: _ },
                DeleteSorter { id, index }
            ) => DeleteSorter { id, index },

            (
                InsertNode { node: _ },
                SetSortColumn { id, index, column }
            ) => SetSortColumn { id, index, column },

            (
                InsertNode { node: _ },
                SetSortDirection { id, index, direction }
            ) => SetSortDirection { id, index, direction },

            (
                InsertIndex { id: _, index: mapper_index },
                InsertIndex { id, index: _ }
            ) => InsertIndex { id, index: *mapper_index },

            (
                InsertIndex { id: _, index: mapper_index },
                DeleteIndex { id, index: _ }
            ) => DeleteIndex { id, index: *mapper_index },

            (
                InsertAggregate { id: _, index: mapper_index, aggregate: _ },
                InsertAggregate { id, index: _, aggregate }
            ) => InsertAggregate { id, index: *mapper_index, aggregate },

            (
                InsertAggregate { id: _, index: mapper_index, aggregate: _ },
                DeleteAggregate { id, index: _ }
            ) => DeleteAggregate { id, index: *mapper_index },

            (
                InsertAggregate { id: _, index: mapper_index, aggregate: _ },
                SetAggregateComputation { id, index: _, computation }
            ) => SetAggregateComputation { id, index: *mapper_index, computation },

            (
                InsertAggregate { id: _, index: mapper_index, aggregate: _ },
                SetAggregateColumn { id, index: _, column }
            ) => SetAggregateColumn { id, index: *mapper_index, column },

            (
                InsertAggregate { id: _, index: mapper_index, aggregate: _ },
                SetAggregateAlias { id, index: _, alias }
            ) => SetAggregateAlias { id, index: *mapper_index, alias },

            (
                InsertFilter { id: _, index: mapper_index, filter: _ },
                InsertFilter { id, index: _, filter }
            ) => InsertFilter { id, index: *mapper_index, filter },

            (
                InsertFilter { id: _, index: mapper_index, filter: _ },
                DeleteFilter { id, index: _ }
            ) => DeleteFilter { id, index: *mapper_index },

            (
                InsertFilter { id: _, index: mapper_index, filter: _ },
                SetFilterColumn { id, index: _, column }
            ) => SetFilterColumn { id, index: *mapper_index, column },

            (
                InsertFilter { id: _, index: mapper_index, filter: _ },
                SetFilterPredicate { id, index: _, predicate }
            ) => SetFilterPredicate { id, index: *mapper_index, predicate },

            (
                InsertJoin { id: _, index: mapper_index, join_column: _ },
                InsertJoin { id, index: _, join_column }
            ) => InsertJoin { id, index: *mapper_index, join_column },

            (
                InsertJoin { id: _, index: mapper_index, join_column: _ },
                DeleteJoin { id, index: _ }
            ) => DeleteJoin { id, index: *mapper_index },

            (
                InsertJoin { id: _, index: mapper_index, join_column: _ },
                SetLeftJoinColumn { id, index: _, column }
            ) => SetLeftJoinColumn { id, index: *mapper_index, column },

            (
                InsertJoin { id: _, index: mapper_index, join_column: _ },
                SetRightJoinColumn { id, index: _, column }
            ) => SetRightJoinColumn { id, index: *mapper_index, column },

            (
                InsertSelect { id: _, index: mapper_index, column: _ },
                InsertSelect { id, index: _, column }
            ) => InsertSelect { id, index: *mapper_index, column },

            (
                InsertSelect { id: _, index: mapper_index, column: _ },
                DeleteSelect { id, index: _ }
            ) => DeleteSelect { id, index: *mapper_index },

            (
                InsertSelect { id: _, index: mapper_index, column: _ },
                SetSelectColumn { id, index: _, column }
            ) => SetSelectColumn { id, index: *mapper_index, column },

            (
                InsertSelect { id: _, index: mapper_index, column: _ },
                SetSelectAlias{ id, index: _, alias }
            ) => SetSelectAlias { id, index: *mapper_index, alias },

            (
                InsertSorter { id: _, index: mapper_index, sorter: _ },
                InsertSorter { id, index: _, sorter }
            ) => InsertSorter { id, index: *mapper_index, sorter },

            (
                InsertSorter { id: _, index: mapper_index, sorter: _ },
                DeleteSorter { id, index: _ }
            ) => DeleteSorter { id, index: *mapper_index },

            (
                InsertSorter { id: _, index: mapper_index, sorter: _ },
                SetSortColumn { id, index: _, column }
            ) => SetSortColumn { id, index: *mapper_index, column },

            (
                InsertSorter { id: _, index: mapper_index, sorter: _ },
                SetSortDirection { id, index: _, direction }
            ) => SetSortDirection { id, index: *mapper_index, direction },

            (a, b) => panic!("Can't map {:?} to {:?}", b, a)
        }
    }

    pub fn transform_backward(self, preceded_by: &Operation) -> Option<Operation> {
        use Operation::*;

        match (preceded_by, self) {
            (
                InsertNode { node: pre_node },
                DeleteNode { id }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(DeleteNode { id })
                }
            }

            (
                InsertNode { node: pre_node },
                SetInput { id, name, input }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetInput { id, name, input })
                }
            }

            (
                InsertNode { node: pre_node },
                SetPosition { id, position }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetPosition { id, position })
                }
            }

            (
                InsertNode { node: pre_node },
                InsertAggregate { id, index, aggregate }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(InsertAggregate { id, index, aggregate })
                }
            }

            (
                InsertNode { node: pre_node },
                DeleteAggregate { id, index }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(DeleteAggregate { id, index })
                }
            }

            (
                InsertNode { node: pre_node },
                SetAggregateComputation { id, index, computation }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetAggregateComputation { id, index, computation })
                }
            }

            (
                InsertNode { node: pre_node },
                SetAggregateColumn { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetAggregateColumn { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetAggregateAlias { id, index, alias }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetAggregateAlias { id, index, alias })
                }
            }

            (
                InsertNode { node: pre_node },
                InsertFilter { id, index, filter }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(InsertFilter { id, index, filter })
                }
            }

            (
                InsertNode { node: pre_node },
                DeleteFilter { id, index }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(DeleteFilter { id, index })
                }
            }

            (
                InsertNode { node: pre_node },
                SetFilterColumn { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetFilterColumn { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetFilterPredicate { id, index, predicate }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetFilterPredicate { id, index, predicate })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCsvPath { id, path }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCsvPath { id, path })
                }
            }

            (
                InsertNode { node: pre_node },
                SetJoinType { id, join_type }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetJoinType { id, join_type })
                }
            }

            (
                InsertNode { node: pre_node },
                InsertJoin { id, index, join_column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(InsertJoin { id, index, join_column })
                }
            }

            (
                InsertNode { node: pre_node },
                DeleteJoin { id, index }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(DeleteJoin { id, index })
                }
            }

            (
                InsertNode { node: pre_node },
                SetLeftJoinColumn { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetLeftJoinColumn { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetRightJoinColumn { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetRightJoinColumn { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                InsertSelect { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(InsertSelect { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                DeleteSelect { id, index }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(DeleteSelect { id, index })
                }
            }

            (
                InsertNode { node: pre_node },
                SetSelectColumn { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetSelectColumn { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetSelectAlias { id, index, alias }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetSelectAlias { id, index, alias })
                }
            }

            (
                InsertNode { node: pre_node },
                InsertSorter { id, index, sorter }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(InsertSorter { id, index, sorter })
                }
            }

            (
                InsertNode { node: pre_node },
                DeleteSorter { id, index }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(DeleteSorter { id, index })
                }
            }

            (
                InsertNode { node: pre_node },
                SetSortColumn { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetSortColumn { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetSortDirection { id, index, direction }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetSortDirection { id, index, direction })
                }
            }

            (
                InsertIndex { id: _, index: pre_index },
                InsertIndex { id, mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(InsertIndex { index, id })
                }
            }

            (
                InsertIndex { id: _, index: pre_index },
                DeleteIndex { id, mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(DeleteIndex { index, id })
                }
            }

            (
                DeleteIndex { id: _, index: pre_index },
                InsertIndex { id, mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(InsertIndex { index, id })
            }

            (
                DeleteIndex { id: _, index: pre_index },
                DeleteIndex { id, mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(DeleteIndex { index, id })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                InsertAggregate { id, mut index, aggregate }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(InsertAggregate { id, index, aggregate })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                DeleteAggregate { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteAggregate { id, index })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                SetAggregateComputation { id, mut index, computation }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetAggregateComputation { id, index, computation })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                SetAggregateColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetAggregateColumn { id, index, column })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                SetAggregateAlias { id, mut index, alias }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetAggregateAlias { id, index, alias })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                InsertAggregate { id, mut index, aggregate }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertAggregate { id, index, aggregate })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                DeleteAggregate { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteAggregate { id, index })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                SetAggregateComputation { id, mut index, computation }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetAggregateComputation { id, index, computation })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                SetAggregateColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetAggregateColumn { id, index, column })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                SetAggregateAlias { id, mut index, alias }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetAggregateAlias { id, index, alias })
            }

            (
                InsertFilter { id: pre_id, index: pre_index, filter: _ },
                InsertFilter { id, mut index, filter }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(InsertFilter { id, index, filter })
            }

            (
                InsertFilter { id: pre_id, index: pre_index, filter: _ },
                DeleteFilter { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteFilter { id, index })
            }

            (
                InsertFilter { id: pre_id, index: pre_index, filter: _ },
                SetFilterColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetFilterColumn { id, index, column })
            }

            (
                InsertFilter { id: pre_id, index: pre_index, filter: _ },
                SetFilterPredicate { id, mut index, predicate }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetFilterPredicate { id, index, predicate })
            }

            (
                DeleteFilter { id: pre_id, index: pre_index },
                InsertFilter { id, mut index, filter }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertFilter { id, index, filter })
            }

            (
                DeleteFilter { id: pre_id, index: pre_index },
                DeleteFilter { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteFilter { id, index })
            }

            (
                DeleteFilter { id: pre_id, index: pre_index },
                SetFilterColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetFilterColumn { id, index, column })
            }

            (
                DeleteFilter { id: pre_id, index: pre_index },
                SetFilterPredicate { id, mut index, predicate }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetFilterPredicate { id, index, predicate })
            }

            (
                InsertJoin { id: pre_id, index: pre_index, join_column: _ },
                InsertJoin { id, mut index, join_column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(InsertJoin { id, index, join_column })
            }

            (
                InsertJoin { id: pre_id, index: pre_index, join_column: _ },
                DeleteJoin { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteJoin { id, index })
            }

            (
                InsertJoin { id: pre_id, index: pre_index, join_column: _ },
                SetLeftJoinColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetLeftJoinColumn { id, index, column })
            }

            (
                InsertJoin { id: pre_id, index: pre_index, join_column: _ },
                SetRightJoinColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetRightJoinColumn { id, index, column })
            }

            (
                DeleteJoin { id: pre_id, index: pre_index },
                InsertJoin { id, mut index, join_column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertJoin { id, index, join_column })
            }

            (
                DeleteJoin { id: pre_id, index: pre_index },
                DeleteJoin { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteJoin { id, index })
            }

            (
                DeleteJoin { id: pre_id, index: pre_index },
                SetLeftJoinColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetLeftJoinColumn { id, index, column })
            }

            (
                DeleteJoin { id: pre_id, index: pre_index },
                SetRightJoinColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetRightJoinColumn { id, index, column })
            }

            (
                InsertSelect { id: pre_id, index: pre_index, column: _ },
                InsertSelect { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(InsertSelect { id, index, column })
            }

            (
                InsertSelect { id: pre_id, index: pre_index, column: _ },
                DeleteSelect { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteSelect { id, index })
            }

            (
                InsertSelect { id: pre_id, index: pre_index, column: _ },
                SetSelectColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetSelectColumn { id, index, column })
            }

            (
                InsertSelect { id: pre_id, index: pre_index, column: _ },
                SetSelectAlias{ id, mut index, alias }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetSelectAlias { id, index, alias })
            }

            (
                DeleteSelect { id: pre_id, index: pre_index },
                InsertSelect { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertSelect { id, index, column })
            }

            (
                DeleteSelect { id: pre_id, index: pre_index },
                DeleteSelect { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteSelect { id, index })
            }

            (
                DeleteSelect { id: pre_id, index: pre_index },
                SetSelectColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetSelectColumn { id, index, column })
            }

            (
                DeleteSelect { id: pre_id, index: pre_index },
                SetSelectAlias { id, mut index, alias }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetSelectAlias { id, index, alias })
            }

            (
                InsertSorter { id: pre_id, index: pre_index, sorter: _ },
                InsertSorter { id, mut index, sorter }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(InsertSorter { id, index, sorter })
            }

            (
                InsertSorter { id: pre_id, index: pre_index, sorter: _ },
                DeleteSorter { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteSorter { id, index })
            }

            (
                InsertSorter { id: pre_id, index: pre_index, sorter: _ },
                SetSortColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetSortColumn { id, index, column })
            }

            (
                InsertSorter { id: pre_id, index: pre_index, sorter: _ },
                SetSortDirection { id, mut index, direction }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetSortDirection { id, index, direction })
            }

            (
                DeleteSorter { id: pre_id, index: pre_index },
                InsertSorter { id, mut index, sorter }
            ) => {
                if  &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertSorter { id, index, sorter })
            }

            (
                DeleteSorter { id: pre_id, index: pre_index },
                DeleteSorter { id, mut index }
            ) => {
                if  &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteSorter { id, index })
            }

            (
                DeleteSorter { id: pre_id, index: pre_index },
                SetSortColumn { id, mut index, column }
            ) => {
                if  &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetSortColumn { id, index, column })
            }

            (
                DeleteSorter { id: pre_id, index: pre_index },
                SetSortDirection { id, mut index, direction }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetSortDirection { id, index, direction })
            }

            (_, op) => Some(op)
        }
    }

    pub fn transform_forward(self, preceded_by: &Operation) -> Option<Operation> {
        use Operation::*;

        match (preceded_by, self) {
            (
                DeleteNode { id: pre_id },
                DeleteNode { id }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DeleteNode { id })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetInput { id, name, input }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetInput { id, name, input })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetPosition { id, position }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetPosition { id, position })
                }
            }

            (
                DeleteNode { id: pre_id },
                InsertAggregate { id, index, aggregate }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(InsertAggregate { id, index, aggregate })
                }
            }

            (
                DeleteNode { id: pre_id },
                DeleteAggregate { id, index }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DeleteAggregate { id, index })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetAggregateComputation { id, index, computation }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetAggregateComputation { id, index, computation })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetAggregateColumn { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetAggregateColumn { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetAggregateAlias { id, index, alias }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetAggregateAlias { id, index, alias })
                }
            }

            (
                DeleteNode { id: pre_id },
                InsertFilter { id, index, filter }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(InsertFilter { id, index, filter })
                }
            }

            (
                DeleteNode { id: pre_id },
                DeleteFilter { id, index }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DeleteFilter { id, index })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetFilterColumn { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetFilterColumn { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetFilterPredicate { id, index, predicate }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetFilterPredicate { id, index, predicate })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCsvPath { id, path }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCsvPath { id, path })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetJoinType { id, join_type }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetJoinType { id, join_type })
                }
            }

            (
                DeleteNode { id: pre_id },
                InsertJoin { id, index, join_column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(InsertJoin { id, index, join_column })
                }
            }

            (
                DeleteNode { id: pre_id },
                DeleteJoin { id, index }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DeleteJoin { id, index })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetLeftJoinColumn { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetLeftJoinColumn { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetRightJoinColumn { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetRightJoinColumn { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                InsertSelect { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(InsertSelect { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                DeleteSelect { id, index }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DeleteSelect { id, index })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetSelectColumn { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetSelectColumn { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetSelectAlias { id, index, alias }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetSelectAlias { id, index, alias })
                }
            }

            (
                DeleteNode { id: pre_id },
                InsertSorter { id, index, sorter }
            ) => {
                if &id == pre_id {
                    None
                } else {
                   Some(InsertSorter { id, index, sorter })
                }
            }

            (
                DeleteNode { id: pre_id },
                DeleteSorter { id, index }
            ) => {
                if &id == pre_id {
                    None
                } else {
                   Some(DeleteSorter { id, index })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetSortColumn { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                   Some(SetSortColumn { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetSortDirection { id, index, direction }
            ) => {
                if &id == pre_id {
                    None
                } else {
                   Some(SetSortDirection { id, index, direction })
                }
            }

            (
                InsertIndex { id: _, index: pre_index },
                InsertIndex { id, mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(InsertIndex { index, id })
            }

            (
                InsertIndex { id: _, index: pre_index },
                DeleteIndex { id, mut index }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(DeleteIndex { index, id })
            }

            (
                DeleteIndex { id: _, index: pre_index },
                InsertIndex { id, mut index }
            ) => {
                if index > *pre_index {
                    index -= 1;
                }
                Some(DeleteIndex { index, id })
            }

            (
                DeleteIndex { id: _, index: pre_index },
                DeleteIndex { id, mut index }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(DeleteIndex { index, id })
                }
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                InsertAggregate { id, mut index, aggregate }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertAggregate { id, index, aggregate })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                DeleteAggregate { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteAggregate { id, index })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                SetAggregateComputation { id, mut index, computation }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetAggregateComputation { id, index, computation })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                SetAggregateColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetAggregateColumn { id, index, column })
            }

            (
                InsertAggregate { id: pre_id, index: pre_index, aggregate: _ },
                SetAggregateAlias { id, mut index, alias }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetAggregateAlias { id, index, alias })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                InsertAggregate { id, mut index, aggregate }
            ) => {
                if &id == pre_id && index > *pre_index {
                    index -= 1;
                }
                Some(InsertAggregate { id, index, aggregate })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                DeleteAggregate { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteAggregate { id, index })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                SetAggregateComputation { id, mut index, computation }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetAggregateComputation { id, index, computation })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                SetAggregateColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetAggregateColumn { id, index, column })
            }

            (
                DeleteAggregate { id: pre_id, index: pre_index },
                SetAggregateAlias { id, mut index, alias }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetAggregateAlias { id, index, alias })
            }

            (
                InsertFilter { id: pre_id, index: pre_index, filter: _ },
                InsertFilter { id, mut index, filter }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertFilter { id, index, filter })
            }

            (
                InsertFilter { id: pre_id, index: pre_index, filter: _ },
                DeleteFilter { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteFilter { id, index })
            }

            (
                InsertFilter { id: pre_id, index: pre_index, filter: _ },
                SetFilterColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetFilterColumn { id, index, column })
            }

            (
                InsertFilter { id: pre_id, index: pre_index, filter: _ },
                SetFilterPredicate { id, mut index, predicate }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetFilterPredicate { id, index, predicate })
            }

            (
                DeleteFilter { id: pre_id, index: pre_index },
                InsertFilter { id, mut index, filter }
            ) => {
                if &id == pre_id && index > *pre_index {
                    index -= 1;
                }
                Some(InsertFilter { id, index, filter })
            }

            (
                DeleteFilter { id: pre_id, index: pre_index },
                DeleteFilter { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteFilter { id, index })
            }

            (
                DeleteFilter { id: pre_id, index: pre_index },
                SetFilterColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetFilterColumn { id, index, column })
            }

            (
                DeleteFilter { id: pre_id, index: pre_index },
                SetFilterPredicate { id, mut index, predicate }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetFilterPredicate { id, index, predicate })
            }

            (
                InsertJoin { id: pre_id, index: pre_index, join_column: _ },
                InsertJoin { id, mut index, join_column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertJoin { id, index, join_column })
            }

            (
                InsertJoin { id: pre_id, index: pre_index, join_column: _ },
                DeleteJoin { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteJoin { id, index })
            }

            (
                InsertJoin { id: pre_id, index: pre_index, join_column: _ },
                SetLeftJoinColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetLeftJoinColumn { id, index, column })
            }

            (
                InsertJoin { id: pre_id, index: pre_index, join_column: _ },
                SetRightJoinColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetRightJoinColumn { id, index, column })
            }

            (
                DeleteJoin { id: pre_id, index: pre_index },
                InsertJoin { id, mut index, join_column }
            ) => {
                if &id == pre_id && index > *pre_index {
                    index -= 1;
                }
                Some(InsertJoin { id, index, join_column })
            }

            (
                DeleteJoin { id: pre_id, index: pre_index },
                DeleteJoin { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteJoin { id, index })
            }

            (
                DeleteJoin { id: pre_id, index: pre_index },
                SetLeftJoinColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetLeftJoinColumn { id, index, column })
            }

            (
                DeleteJoin { id: pre_id, index: pre_index },
                SetRightJoinColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetRightJoinColumn { id, index, column })
            }

            (
                InsertSelect { id: pre_id, index: pre_index, column: _ },
                InsertSelect { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertSelect { id, index, column })
            }

            (
                InsertSelect { id: pre_id, index: pre_index, column: _ },
                DeleteSelect { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteSelect { id, index })
            }

            (
                InsertSelect { id: pre_id, index: pre_index, column: _ },
                SetSelectColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetSelectColumn { id, index, column })
            }

            (
                InsertSelect { id: pre_id, index: pre_index, column: _ },
                SetSelectAlias { id, mut index, alias }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetSelectAlias { id, index, alias })
            }

            (
                DeleteSelect { id: pre_id, index: pre_index },
                InsertSelect { id, mut index, column }
            ) => {
                if &id == pre_id && index > *pre_index {
                    index -= 1;
                }
                Some(InsertSelect { id, index, column })
            }

            (
                DeleteSelect { id: pre_id, index: pre_index },
                DeleteSelect { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteSelect { id, index })
            }

            (
                DeleteSelect { id: pre_id, index: pre_index },
                SetSelectColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetSelectColumn { id, index, column })
            }

            (
                DeleteSelect { id: pre_id, index: pre_index },
                SetSelectAlias { id, mut index, alias }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetSelectAlias { id, index, alias })
            }

            (
                InsertSorter { id: pre_id, index: pre_index, sorter: _ },
                InsertSorter { id, mut index, sorter }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertSorter { id, index, sorter })
            }

            (
                InsertSorter { id: pre_id, index: pre_index, sorter: _ },
                DeleteSorter { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteSorter { id, index })
            }

            (
                InsertSorter { id: pre_id, index: pre_index, sorter: _ },
                SetSortColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetSortColumn { id, index, column })
            }

            (
                InsertSorter { id: pre_id, index: pre_index, sorter: _ },
                SetSortDirection { id, mut index, direction }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetSortDirection { id, index, direction })
            }

            (
                DeleteSorter { id: pre_id, index: pre_index },
                InsertSorter { id, mut index, sorter }
            ) => {
                if &id == pre_id && index > *pre_index {
                    index -= 1;
                }
                Some(InsertSorter { id, index, sorter })
            }

            (
                DeleteSorter { id: pre_id, index: pre_index },
                DeleteSorter { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteSorter { id, index })
            }

            (
                DeleteSorter { id: pre_id, index: pre_index },
                SetSortColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetSortColumn { id, index, column })
            }

            (
                DeleteSorter { id: pre_id, index: pre_index },
                SetSortDirection { id, mut index, direction }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetSortDirection { id, index, direction })
            }

            (_, op) => Some(op)
        }
    }
}

pub fn transform_batch(batch: Vec<Operation>, preceded_by: &Vec<Operation>) -> Vec<Operation> {
    let mut transformed_batch: Vec<Option<Operation>> = Vec::with_capacity(batch.len());

    batch.iter()
        .enumerate()
        .for_each(|(i, operation)| {
            let prevs = &batch[..i];
            let mut transformed = Some(operation.clone());
            let mut forward_from_index_in_batch = 0;

            // Transform backward against previous operations in the batch.
            for (j, prev_op) in prevs.iter().enumerate().rev() {
                let op = transformed.take().unwrap();
                if let Some(op) = op.transform_backward(prev_op) {
                    transformed = Some(op);
                } else {
                    if let Some(prev_transformed) = &transformed_batch[j].as_ref() {
                        transformed = Some(operation.clone().map(prev_transformed));
                        forward_from_index_in_batch = j + 1;
                    } else {
                        transformed = None;
                    }
                    break;
                }
            }

            // Transform forward against preceding operations.
            if forward_from_index_in_batch == 0 {
                for preceding in preceded_by.iter() {
                    if let Some(op) = transformed.take() {
                        transformed = op.transform_forward(preceding);
                    } else {
                        transformed_batch.push(None);
                        return;
                    }
                }
            }

            // Transform forward against transformed operations in the batch.
            for prev in transformed_batch[forward_from_index_in_batch..].iter() {
                if let Some(op) = transformed.take() {
                    if let Some(prev) = prev {
                        transformed = op.transform_forward(prev);
                    } else {
                        continue;
                    }
                } else {
                    transformed_batch.push(None);
                    return;
                }
            }

            transformed_batch.push(transformed);
        });

    // Remove dropped operations.
    transformed_batch.into_iter()
        .filter(|op| op.is_some())
        .map(|op| op.unwrap())
        .collect()
}

pub fn validate_sequence(operations: &Vec<Operation>) -> Result<(), PoldaError> {
    // DeleteIndex must be followed by DeleteNode or InsertIndex.
    // InsertNode must be followed by InsertIndex.
    let mut sequence: Option<&Operation> = None;

    for op in operations.iter() {
        use Operation::*;
        match sequence {
            Some(DeleteIndex { id: prev_id, index: _ }) => {
                let err = "DeleteIndex operation must be followed by DeleteNode or InsertIndex operation with the same node id";
                match op {
                    DeleteNode { id } => {
                        if id == prev_id {
                            sequence = None
                        } else {
                            return Err(PoldaError::OperationError(err.to_string()));
                        }
                    }
                    InsertIndex { id, index: _ } => {
                        if id == prev_id {
                            sequence = None
                        } else {
                            return Err(PoldaError::OperationError(err.to_string()));
                        }
                    }
                    _ => {
                        return Err(PoldaError::OperationError(err.to_string()));
                    }
                }
            }
            Some(InsertNode { node }) => {
                let err = "InsertNode operation must be followed by InsertIndex operation with the same node id";
                if let InsertIndex { id, index: _ } = op {
                    if id == node.id() {
                        sequence = None;
                    } else {
                        return Err(PoldaError::OperationError(err.to_string()));
                    }
                } else {
                    return Err(PoldaError::OperationError(err.to_string()));
                }
            }
            _ => {
                match op {
                    InsertNode { node: _ } => {
                        sequence = Some(op);
                    }
                    DeleteIndex { id: _, index: _ } => {
                        sequence = Some(op);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
