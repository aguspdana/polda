use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

use super::Aggregate;
use super::Filter;
use super::JoinColumn;
use super::JoinType;
use super::Position;
use super::SelectColumn;
use super::Sorter;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Node {
    Aggregate {
        id: String,
        position: Position,
        input: Option<String>,
        aggregates: Vec<Aggregate>,
        outputs: HashSet<String>
    },
    Filter {
        id: String,
        position: Position,
        input: Option<String>,
        filters: Vec<Filter>,
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
        path: String,
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

            Filter {
                id,
                position: _,
                input: _,
                filters: _,
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
                path: _,
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

            Filter {
                id: _,
                position: _,
                input,
                filters: _,
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
                path: _,
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

            Filter {
                id: _,
                position: _,
                input: _,
                filters: _,
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
                path: _,
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

            Filter {
                id: _,
                position: _,
                input: _,
                filters: _,
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
                path: _,
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

            Filter {
                id: _,
                position: _,
                input: _,
                filters: _,
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
                path: _,
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
