use serde::Deserialize;
use serde::Serialize;

use crate::data_type::DataType;
use crate::error::PoldaError;
use super::Node;
use super::InputName;
use super::Position;
use super::Aggregate;
use super::AggregateComputation;
use super::FilterPredicate;
use super::JoinColumn;
use super::JoinType;
use super::SelectColumn;
use super::SortDirection;
use super::Sorter;
use super::Value;
use super::types::case::Case;
use super::types::compute::ComputeOperation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
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

    // Bins node operations
    SetBinsName {
        id: String,
        name: String
    },

    SetBinsColumn {
        id: String,
        column: String
    },

    SetBinsLowerBound {
        id: String,
        lower_bound: f64
    },

    SetBinsUpperBound {
        id: String,
        upper_bound: f64
    },

    SetBinsCount {
        id: String,
        count: usize
    },

    // Case node operations
    SetCaseName {
        id: String,
        name: String
    },

    SetCaseDataType {
        id: String,
        data_type: DataType
    },

    InsertCase {
        id: String,
        index: usize,
        case: Case
    },

    DeleteCase {
        id: String,
        index: usize
    },

    SetCaseColumn {
        id: String,
        index: usize,
        column: String
    },

    SetCaseValue {
        id: String,
        index: usize,
        value: Value
    },

    SetCaseDefault {
        id: String,
        default: Value
    },

    // Cast node operations
    SetCastName {
        id: String,
        name: String
    },

    SetCastColumn {
        id: String,
        column: String
    },

    SetCastDataType {
        id: String,
        data_type: DataType
    },

    // Compute node operations
    SetComputeName {
        id: String,
        name: String
    },

    SetComputeColumn {
        id: String,
        column: String
    },

    SetComputeOperation {
        id: String,
        operation: ComputeOperation
    },

    // Filter node operations
    SetFilterColumn {
        id: String,
        column: String
    },
    SetFilterPredicate {
        id: String,
        predicate: FilterPredicate
    },

    // LoadCsv node operations:
    SetLoadCsvFilename {
        id: String,
        filename: String
    },

    // Join node operations:
    SetJoinType {
        id: String,
        join_type: JoinType
    },
    InsertJoinColumn {
        id: String,
        index: usize,
        join_column: JoinColumn
    },
    DeleteJoinColumn {
        id: String,
        index: usize
    },
    SetJoinColumnLeft {
        id: String,
        index: usize,
        column: String
    },
    SetJoinColumnRight {
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

            // Aggregate node operations

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

            // Bins node operations

            SetBinsName {
                id,
                name: _
            } => id,

            SetBinsColumn {
                id,
                column: _
            } => id,

            SetBinsLowerBound {
                id,
                lower_bound: _
            } => id,

            SetBinsUpperBound {
                id,
                upper_bound: _
            } => id,

            SetBinsCount {
                id,
                count: _
            } => id,

            // Case node operations
            SetCaseName {
                id,
                name: _
            } => id,

            SetCaseDataType {
                id,
                data_type: _
            } => id,

            InsertCase {
                id,
                index: _,
                case: _
            } => id,

            DeleteCase {
                id,
                index: _
            } => id,

            SetCaseColumn {
                id,
                index: _,
                column: _
            } => id,

            SetCaseValue {
                id,
                index: _,
                value: _
            } => id,

            SetCaseDefault {
                id,
                default: _
            } => id,

            // Cast node operations
            SetCastName {
                id,
                name: _
            } => id,

            SetCastColumn {
                id,
                column: _
            } => id,

            SetCastDataType {
                id,
                data_type: _
            } => id,

            // Compute node operations
            SetComputeName {
                id,
                name: _
            } => id,

            SetComputeColumn {
                id,
                column: _
            } => id,

            SetComputeOperation {
                id,
                operation: _
            } => id,

            // Filter node operations

            SetFilterColumn {
                id,
                column: _
            } => id,

            SetFilterPredicate {
                id,
                predicate: _
            } => id,

            // LoadCsv node operations

            SetLoadCsvFilename {
                id,
                filename: _
            } => id,

            // Join node operations

            SetJoinType {
                id,
                join_type: _
            } => id,

            InsertJoinColumn {
                id,
                index: _,
                join_column: _
            } => id,

            DeleteJoinColumn {
                id,
                index: _
            } => id,

            SetJoinColumnLeft {
                id,
                index: _,
                column: _
            } => id,

            SetJoinColumnRight {
                id,
                index: _,
                column: _
            } => id,

            // Select node operations

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

            // Sort node operations

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
                SetBinsName { id, name }
            ) => SetBinsName { id, name },

            (
                InsertNode { node: _ },
                SetBinsColumn { id, column }
            ) => SetBinsColumn { id, column },

            (
                InsertNode { node: _ },
                SetBinsLowerBound { id, lower_bound }
            ) => SetBinsLowerBound { id, lower_bound },

            (
                InsertNode { node: _ },
                SetBinsUpperBound { id, upper_bound }
            ) => SetBinsUpperBound { id, upper_bound },

            (
                InsertNode { node: _ },
                SetBinsCount { id, count }
            ) => SetBinsCount { id, count },

            (
                InsertNode { node: _ },
                SetCaseName { id, name }
            ) => SetBinsName { id, name },

            (
                InsertNode { node: _ },
                SetCaseDataType { id, data_type }
            ) => SetCaseDataType { id, data_type },

            (
                InsertNode { node: _ },
                InsertCase { id, index, case }
            ) => InsertCase { id, index, case },

            (
                InsertNode { node: _ },
                DeleteCase { id, index }
            ) => DeleteCase { id, index },

            (
                InsertNode { node: _ },
                SetCaseColumn { id, index, column }
            ) => SetCaseColumn { id, index, column },

            (
                InsertNode { node: _ },
                SetCaseValue { id, index, value }
            ) => SetCaseValue { id, index, value },

            (
                InsertNode { node: _ },
                SetCaseDefault { id, default }
            ) => SetCaseDefault { id, default },

            (
                InsertNode { node: _ },
                SetCastName { id, name }
            ) => SetCastName { id, name },

            (
                InsertNode { node: _ },
                SetCastColumn { id, column }
            ) => SetCastColumn { id, column },

            (
                InsertNode { node: _ },
                SetCastDataType { id, data_type }
            ) => SetCastDataType { id, data_type },

            (
                InsertNode { node: _ },
                SetComputeName { id, name }
            ) => SetComputeName { id, name },

            (
                InsertNode { node: _ },
                SetComputeColumn { id, column }
            ) => SetComputeColumn { id, column },

            (
                InsertNode { node: _ },
                SetComputeOperation { id, operation }
            ) => SetComputeOperation { id, operation },

            (
                InsertNode { node: _ },
                SetFilterColumn { id, column }
            ) => SetFilterColumn { id, column },

            (
                InsertNode { node: _ },
                SetFilterPredicate { id, predicate }
            ) => SetFilterPredicate { id, predicate },

            (
                InsertNode { node: _ },
                SetLoadCsvFilename { id, filename }
            ) => SetLoadCsvFilename { id, filename },

            (
                InsertNode { node: _ },
                SetJoinType { id, join_type }
            ) => SetJoinType { id, join_type },

            (
                InsertNode { node: _ },
                InsertJoinColumn { id, index, join_column }
            ) => InsertJoinColumn { id, index, join_column },

            (
                InsertNode { node: _ },
                DeleteJoinColumn { id, index }
            ) => DeleteJoinColumn { id, index },

            (
                InsertNode { node: _ },
                SetJoinColumnLeft { id, index, column }
            ) => SetJoinColumnLeft { id, index, column },

            (
                InsertNode { node: _ },
                SetJoinColumnRight { id, index, column }
            ) => SetJoinColumnRight { id, index, column },

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
                InsertCase { id: _, index: mapper_index, case: _ },
                InsertCase { id, index: _, case }
            ) => InsertCase { id, index: *mapper_index, case },

            (
                InsertCase { id: _, index: mapper_index, case: _ },
                DeleteCase { id, index: _ }
            ) => DeleteCase { id, index: *mapper_index },

            (
                InsertCase { id: _, index: mapper_index, case: _ },
                SetCaseColumn { id, index: _, column }
            ) => SetCaseColumn { id, index: *mapper_index, column },

            (
                InsertCase { id: _, index: mapper_index, case: _ },
                SetCaseValue { id, index: _, value }
            ) => SetCaseValue { id, index: *mapper_index, value },

            (
                InsertJoinColumn { id: _, index: mapper_index, join_column: _ },
                InsertJoinColumn { id, index: _, join_column }
            ) => InsertJoinColumn { id, index: *mapper_index, join_column },

            (
                InsertJoinColumn { id: _, index: mapper_index, join_column: _ },
                DeleteJoinColumn { id, index: _ }
            ) => DeleteJoinColumn { id, index: *mapper_index },

            (
                InsertJoinColumn { id: _, index: mapper_index, join_column: _ },
                SetJoinColumnLeft { id, index: _, column }
            ) => SetJoinColumnLeft { id, index: *mapper_index, column },

            (
                InsertJoinColumn { id: _, index: mapper_index, join_column: _ },
                SetJoinColumnRight { id, index: _, column }
            ) => SetJoinColumnRight { id, index: *mapper_index, column },

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
                SetBinsName { id, name }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetBinsName { id, name })
                }
            }

            (
                InsertNode { node: pre_node },
                SetBinsColumn { id, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetBinsColumn { id, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetBinsLowerBound { id, lower_bound }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetBinsLowerBound { id, lower_bound })
                }
            }

            (
                InsertNode { node: pre_node },
                SetBinsUpperBound { id, upper_bound }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetBinsUpperBound { id, upper_bound })
                }
            }

            (
                InsertNode { node: pre_node },
                SetBinsCount { id, count }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetBinsCount { id, count })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCaseName { id, name }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCaseName { id, name })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCaseDataType { id, data_type }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCaseDataType { id, data_type })
                }
            }

            (
                InsertNode { node: pre_node },
                InsertCase { id, index, case }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(InsertCase { id, index, case })
                }
            }

            (
                InsertNode { node: pre_node },
                DeleteCase { id, index }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(DeleteCase { id, index })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCaseColumn { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCaseColumn { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCaseValue { id, index, value }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCaseValue { id, index, value })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCaseDefault { id, default }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCaseDefault { id, default })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCastName { id, name }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCastName { id, name })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCastColumn { id, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCastColumn { id, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetCastDataType { id, data_type }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetCastDataType { id, data_type })
                }
            }

            (
                InsertNode { node: pre_node },
                SetComputeName { id, name }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetComputeName { id, name })
                }
            }

            (
                InsertNode { node: pre_node },
                SetComputeColumn { id, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetComputeColumn { id, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetComputeOperation { id, operation }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetComputeOperation { id, operation })
                }
            }

            (
                InsertNode { node: pre_node },
                SetFilterColumn { id, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetFilterColumn { id, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetFilterPredicate { id, predicate }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetFilterPredicate { id, predicate })
                }
            }

            (
                InsertNode { node: pre_node },
                SetLoadCsvFilename { id, filename }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetLoadCsvFilename { id, filename })
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
                InsertJoinColumn { id, index, join_column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(InsertJoinColumn { id, index, join_column })
                }
            }

            (
                InsertNode { node: pre_node },
                DeleteJoinColumn { id, index }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(DeleteJoinColumn { id, index })
                }
            }

            (
                InsertNode { node: pre_node },
                SetJoinColumnLeft { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetJoinColumnLeft { id, index, column })
                }
            }

            (
                InsertNode { node: pre_node },
                SetJoinColumnRight { id, index, column }
            ) => {
                if &id == pre_node.id() {
                    None
                } else {
                    Some(SetJoinColumnRight { id, index, column })
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
                InsertCase { id: pre_id, index: pre_index, case: _ },
                InsertCase { id, mut index, case }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(InsertCase { id, index, case })
            }

            (
                InsertCase { id: pre_id, index: pre_index, case: _ },
                DeleteCase { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteCase { id, index })
            }

            (
                InsertCase { id: pre_id, index: pre_index, case: _ },
                SetCaseColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetCaseColumn { id, index, column })
            }

            (
                InsertCase { id: pre_id, index: pre_index, case: _ },
                SetCaseValue { id, mut index, value }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetCaseValue { id, index, value })
            }

            (
                DeleteCase { id: pre_id, index: pre_index },
                InsertCase { id, mut index, case }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertCase { id, index, case })
            }

            (
                DeleteCase { id: pre_id, index: pre_index },
                DeleteCase { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteCase { id, index })
            }

            (
                DeleteCase { id: pre_id, index: pre_index },
                SetCaseColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetCaseColumn { id, index, column })
            }

            (
                DeleteCase { id: pre_id, index: pre_index },
                SetCaseValue { id, mut index, value }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetCaseValue { id, index, value })
            }

            (
                InsertJoinColumn { id: pre_id, index: pre_index, join_column: _ },
                InsertJoinColumn { id, mut index, join_column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(InsertJoinColumn { id, index, join_column })
            }

            (
                InsertJoinColumn { id: pre_id, index: pre_index, join_column: _ },
                DeleteJoinColumn { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteJoinColumn { id, index })
            }

            (
                InsertJoinColumn { id: pre_id, index: pre_index, join_column: _ },
                SetJoinColumnLeft { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetJoinColumnLeft { id, index, column })
            }

            (
                InsertJoinColumn { id: pre_id, index: pre_index, join_column: _ },
                SetJoinColumnRight { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetJoinColumnRight { id, index, column })
            }

            (
                DeleteJoinColumn { id: pre_id, index: pre_index },
                InsertJoinColumn { id, mut index, join_column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertJoinColumn { id, index, join_column })
            }

            (
                DeleteJoinColumn { id: pre_id, index: pre_index },
                DeleteJoinColumn { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteJoinColumn { id, index })
            }

            (
                DeleteJoinColumn { id: pre_id, index: pre_index },
                SetJoinColumnLeft { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetJoinColumnLeft { id, index, column })
            }

            (
                DeleteJoinColumn { id: pre_id, index: pre_index },
                SetJoinColumnRight { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetJoinColumnRight { id, index, column })
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
                SetBinsName { id, name }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetBinsName { id, name })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetBinsColumn { id, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetBinsColumn { id, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetBinsLowerBound { id, lower_bound }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetBinsLowerBound { id, lower_bound })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetBinsUpperBound { id, upper_bound }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetBinsUpperBound { id, upper_bound })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetBinsCount { id, count }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetBinsCount { id, count })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCaseName { id, name }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCaseName { id, name })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCaseDataType { id, data_type }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCaseDataType { id, data_type })
                }
            }

            (
                DeleteNode { id: pre_id },
                InsertCase { id, index, case }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(InsertCase { id, index, case })
                }
            }

            (
                DeleteNode { id: pre_id },
                DeleteCase { id, index }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DeleteCase { id, index })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCaseColumn { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCaseColumn { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCaseValue { id, index, value }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCaseValue { id, index, value })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCaseDefault { id, default }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCaseDefault { id, default })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCastName { id, name }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCastName { id, name })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCastColumn { id, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCastColumn { id, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetCastDataType { id, data_type }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetCastDataType { id, data_type })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetComputeName { id, name }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetComputeName { id, name })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetComputeColumn { id, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetComputeColumn { id, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetComputeOperation { id, operation }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetComputeOperation { id, operation })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetFilterColumn { id, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetFilterColumn { id, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetFilterPredicate { id, predicate }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetFilterPredicate { id, predicate })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetLoadCsvFilename { id, filename }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetLoadCsvFilename { id, filename })
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
                InsertJoinColumn { id, index, join_column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(InsertJoinColumn { id, index, join_column })
                }
            }

            (
                DeleteNode { id: pre_id },
                DeleteJoinColumn { id, index }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DeleteJoinColumn { id, index })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetJoinColumnLeft { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetJoinColumnLeft { id, index, column })
                }
            }

            (
                DeleteNode { id: pre_id },
                SetJoinColumnRight { id, index, column }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(SetJoinColumnRight { id, index, column })
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
                InsertCase { id: pre_id, index: pre_index, case: _ },
                InsertCase { id, mut index, case }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertCase { id, index, case })
            }

            (
                InsertCase { id: pre_id, index: pre_index, case: _ },
                DeleteCase { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteCase { id, index })
            }

            (
                InsertCase { id: pre_id, index: pre_index, case: _ },
                SetCaseColumn { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetCaseColumn { id, index, column })
            }

            (
                InsertCase { id: pre_id, index: pre_index, case: _ },
                SetCaseValue { id, mut index, value }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetCaseValue { id, index, value })
            }

            (
                DeleteCase { id: pre_id, index: pre_index },
                InsertCase { id, mut index, case }
            ) => {
                if &id == pre_id && index > *pre_index {
                    index -= 1;
                }
                Some(InsertCase { id, index, case })
            }

            (
                DeleteCase { id: pre_id, index: pre_index },
                DeleteCase { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteCase { id, index })
            }

            (
                DeleteCase { id: pre_id, index: pre_index },
                SetCaseColumn { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetCaseColumn { id, index, column })
            }

            (
                DeleteCase { id: pre_id, index: pre_index },
                SetCaseValue { id, mut index, value }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetCaseValue { id, index, value })
            }

            (
                InsertJoinColumn { id: pre_id, index: pre_index, join_column: _ },
                InsertJoinColumn { id, mut index, join_column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(InsertJoinColumn { id, index, join_column })
            }

            (
                InsertJoinColumn { id: pre_id, index: pre_index, join_column: _ },
                DeleteJoinColumn { id, mut index }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(DeleteJoinColumn { id, index })
            }

            (
                InsertJoinColumn { id: pre_id, index: pre_index, join_column: _ },
                SetJoinColumnLeft { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetJoinColumnLeft { id, index, column })
            }

            (
                InsertJoinColumn { id: pre_id, index: pre_index, join_column: _ },
                SetJoinColumnRight { id, mut index, column }
            ) => {
                if &id == pre_id && index >= *pre_index {
                    index += 1;
                }
                Some(SetJoinColumnRight { id, index, column })
            }

            (
                DeleteJoinColumn { id: pre_id, index: pre_index },
                InsertJoinColumn { id, mut index, join_column }
            ) => {
                if &id == pre_id && index > *pre_index {
                    index -= 1;
                }
                Some(InsertJoinColumn { id, index, join_column })
            }

            (
                DeleteJoinColumn { id: pre_id, index: pre_index },
                DeleteJoinColumn { id, mut index }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(DeleteJoinColumn { id, index })
            }

            (
                DeleteJoinColumn { id: pre_id, index: pre_index },
                SetJoinColumnLeft { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetJoinColumnLeft { id, index, column })
            }

            (
                DeleteJoinColumn { id: pre_id, index: pre_index },
                SetJoinColumnRight { id, mut index, column }
            ) => {
                if &id == pre_id {
                    if index == *pre_index {
                        return None;
                    } else if index > *pre_index {
                        index -= 1;
                    }
                }
                Some(SetJoinColumnRight { id, index, column })
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

pub fn transform_batch(batch: Vec<Operation>, preceded_by: &[Operation]) -> Vec<Operation> {
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
