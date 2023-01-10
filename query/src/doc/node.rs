use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

use crate::data_type::DataType;

use super::Aggregate;
use super::FilterPredicate;
use super::JoinColumn;
use super::JoinType;
use super::Position;
use super::SelectColumn;
use super::Sorter;
use super::Value;
use super::types::case::Case;
use super::types::compute::ComputeOperation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Node {
    Aggregate {
        id: String,
        position: Position,
        input: Option<String>,
        aggregates: Vec<Aggregate>,
        outputs: HashSet<String>
    },
    Bins {
        id: String,
        position: Position,
        input: Option<String>,
        name: String,
        column: String,
        lower_bound: f64,
        upper_bound: f64,
        count: usize,
        outputs: HashSet<String>
    },
    Case {
        id: String,
        position: Position,
        input: Option<String>,
        name: String,
        data_type: DataType,
        cases: Vec<Case>,
        default: Value,
        outputs: HashSet<String>
    },
    Cast {
        id: String,
        position: Position,
        input: Option<String>,
        name: String,
        column: String,
        data_type: DataType,
        outputs: HashSet<String>
    },
    Compute {
        id: String,
        position: Position,
        input: Option<String>,
        name: String,
        column: String,
        operation: ComputeOperation,
        outputs: HashSet<String>
    },
    Filter {
        id: String,
        position: Position,
        input: Option<String>,
        column: String,
        predicate: FilterPredicate,
        outputs: HashSet<String>
    },
    Join {
        id: String,
        position: Position,
        left_input: Option<String>,
        right_input: Option<String>,
        join_type: JoinType,
        columns: Vec<JoinColumn>,
        outputs: HashSet<String>
    },
    LoadCsv {
        id: String,
        position: Position,
        filename: String,
        outputs: HashSet<String>
    },
    Select {
        id: String,
        position: Position,
        input: Option<String>,
        columns: Vec<SelectColumn>,
        outputs: HashSet<String>
    },
    Sort {
        id: String,
        position: Position,
        input: Option<String>,
        sorters: Vec<Sorter>,
        outputs: HashSet<String>
    },
    Union {
        id: String,
        position: Position,
        primary_input: Option<String>,
        secondary_input: Option<String>,
        outputs: HashSet<String>
    }
}

impl Node {
    pub fn id(&self) -> &String {
        use Node::*;

        match self {
            Aggregate {
                id,
                position: _,
                input: _,
                aggregates: _,
                outputs: _
            } => id,

            Bins {
                id,
                position: _,
                input: _,
                name: _,
                column: _,
                lower_bound: _,
                upper_bound: _,
                count: _,
                outputs: _
            } => id,

            Case {
                id,
                position: _,
                input: _,
                name: _,
                data_type: _,
                cases: _,
                default: _,
                outputs: _
            } => id,

            Cast {
                id,
                position: _,
                input: _,
                name: _,
                column: _,
                data_type: _,
                outputs: _
            } => id,

            Compute {
                id,
                position: _,
                input: _,
                name: _,
                column: _,
                operation: _,
                outputs: _
            } => id,

            Filter {
                id,
                position: _,
                input: _,
                column: _,
                predicate: _,
                outputs: _
            } => id,

            Join {
                id,
                position: _,
                left_input: _,
                right_input: _,
                join_type: _,
                columns: _,
                outputs: _
            } => id,

            LoadCsv {
                id,
                position: _,
                filename: _,
                outputs: _
            } => id,

            Select {
                id,
                position: _,
                input: _,
                columns: _,
                outputs: _
            } => id,

            Sort {
                id,
                position: _,
                input: _,
                sorters: _,
                outputs: _
            } => id,

            Union {
                id,
                primary_input: _,
                secondary_input: _,
                position: _,
                outputs: _
            } => id
        }
    }

    pub fn inputs(&self) -> Vec<&Option<String>> {
        use Node::*;

        match self {
            Aggregate {
                id: _,
                position: _,
                input,
                aggregates: _,
                outputs: _
            } => vec![input],

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
            } => vec![input],

            Case {
                id: _,
                position: _,
                input,
                name: _,
                data_type: _,
                cases: _,
                default: _,
                outputs: _
            } => vec![input],

            Cast {
                id: _,
                position: _,
                input,
                name: _,
                column: _,
                data_type: _,
                outputs: _
            } => vec![input],

            Compute {
                id: _,
                position: _,
                input,
                name: _,
                column: _,
                operation: _,
                outputs: _
            } => vec![input],

            Filter {
                id: _,
                position: _,
                input,
                column: _,
                predicate: _,
                outputs: _
            } => vec![input],

            Join {
                id: _,
                position: _,
                left_input,
                right_input,
                join_type: _,
                columns: _,
                outputs: _
            } => vec![left_input, right_input],

            LoadCsv {
                id: _,
                position: _,
                filename: _,
                outputs: _
            } => vec![],

            Select {
                id: _,
                position: _,
                input,
                columns: _,
                outputs: _
            } => vec![input],

            Sort {
                id: _,
                position: _,
                input,
                sorters: _,
                outputs: _
            } => vec![input],

            Union {
                id: _,
                primary_input,
                secondary_input,
                position: _,
                outputs: _
            } => vec![primary_input, secondary_input]
        }
    }

    pub fn insert_output(&mut self, id: String) {
        use Node::*;
        match self {
            Aggregate {
                id: _,
                position: _,
                input: _,
                aggregates: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Bins {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                lower_bound: _,
                upper_bound: _,
                count: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Case {
                id: _,
                position: _,
                input: _,
                name: _,
                data_type: _,
                cases: _,
                default: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Cast {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                data_type: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Compute {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                operation: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Filter {
                id: _,
                position: _,
                input: _,
                column: _,
                predicate: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Join {
                id: _,
                position: _,
                left_input: _,
                right_input: _,
                join_type: _,
                columns: _,
                outputs
            } => {
                outputs.insert(id);
            }

            LoadCsv {
                id: _,
                position: _,
                filename: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Select {
                id: _,
                position: _,
                input: _,
                columns: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Sort {
                id: _,
                position: _,
                input: _,
                sorters: _,
                outputs
            } => {
                outputs.insert(id);
            }

            Union {
                id: _,
                position: _,
                primary_input: _,
                secondary_input: _,
                outputs
            } => {
                outputs.insert(id);
            }
        }
    }

    pub fn outputs(&self) -> &HashSet<String> {
        use Node::*;

        match self {
            Aggregate {
                id: _,
                position: _,
                input: _,
                aggregates: _,
                outputs
            } => outputs,

            Bins {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                lower_bound: _,
                upper_bound: _,
                count: _,
                outputs
            } => outputs,

            Case {
                id: _,
                position: _,
                input: _,
                name: _,
                data_type: _,
                cases: _,
                default: _,
                outputs
            } => outputs,

            Cast {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                data_type: _,
                outputs
            } => outputs,

            Compute {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                operation: _,
                outputs
            } => outputs,

            Filter {
                id: _,
                position: _,
                input: _,
                column: _,
                predicate: _,
                outputs
            } => outputs,

            Join {
                id: _,
                position: _,
                left_input: _,
                right_input: _,
                join_type: _,
                columns: _,
                outputs
            } => outputs,

            LoadCsv {
                id: _,
                position: _,
                filename: _,
                outputs
            } => outputs,

            Select {
                id: _,
                position: _,
                input: _,
                columns: _,
                outputs
            } => outputs,

            Sort {
                id: _,
                position: _,
                input: _,
                sorters: _,
                outputs
            } => outputs,

            Union {
                id: _,
                primary_input: _,
                secondary_input: _,
                position: _,
                outputs
            } => outputs
        }
    }

    pub fn remove_output(&mut self, id: &String) {
        use Node::*;
        match self {
            Aggregate {
                id: _,
                position: _,
                input: _,
                aggregates: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Bins {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                lower_bound: _,
                upper_bound: _,
                count: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Case {
                id: _,
                position: _,
                input: _,
                name: _,
                data_type: _,
                cases: _,
                default: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Cast {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                data_type: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Compute {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                operation: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Filter {
                id: _,
                position: _,
                input: _,
                column: _,
                predicate: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Join {
                id: _,
                position: _,
                left_input: _,
                right_input: _,
                join_type: _,
                columns: _,
                outputs
            } => {
                outputs.remove(id);
            }

            LoadCsv {
                id: _,
                position: _,
                filename: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Select {
                id: _,
                position: _,
                input: _,
                columns: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Sort {
                id: _,
                position: _,
                input: _,
                sorters: _,
                outputs
            } => {
                outputs.remove(id);
            }

            Union {
                id: _,
                position: _,
                primary_input: _,
                secondary_input: _,
                outputs
            } => {
                outputs.remove(id);
            }
        }
    }
}
