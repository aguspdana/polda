use serde::Deserialize;
use serde::Serialize;

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
    Constant {
        id: String,
        position: Position,
        name: String,
        data_type: DataType,
        value: String,
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
