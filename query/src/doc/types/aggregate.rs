use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregate {
    pub column: String,
    pub computation: AggregateComputation,
    pub alias: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregateComputation {
    Count,
    Group,
    Max,
    Mean,
    Median,
    Min,
    Sum,
}
