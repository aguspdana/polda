use super::AggregateComputation;

#[derive(Debug, Clone)]
pub struct Aggregate {
    pub column: String,
    pub computation: AggregateComputation,
    pub alias: String
}
