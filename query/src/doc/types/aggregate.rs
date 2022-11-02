#[derive(Debug, Clone)]
pub struct Aggregate {
    pub column: String,
    pub computation: AggregateComputation,
    pub alias: String
}

#[derive(Debug, Clone)]
pub enum AggregateComputation {
    Count,
    Group,
    Max,
    Mean,
    Median,
    Min,
    Sum,
}
