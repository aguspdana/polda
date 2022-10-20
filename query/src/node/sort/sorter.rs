use super::SortDirection;

#[derive(Debug, Clone)]
pub struct Sorter {
    pub column: String,
    pub direction: SortDirection
}
