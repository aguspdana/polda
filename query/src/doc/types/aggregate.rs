#[derive(Debug, Clone)]
pub struct Aggregate {
    pub column: String,
    pub computation: AggregateComputation,
    pub alias: String
}

#[derive(Debug, Clone)]
pub enum AggregateComputation {
    Avg,
    Count,
    Group,
    Max,
    Median,
    Min,
    Sum,
}
