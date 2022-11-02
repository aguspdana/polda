#[derive(Debug, Clone)]
pub struct Sorter {
    pub column: String,
    pub direction: SortDirection
}

#[derive(Debug, Clone)]
pub enum SortDirection {
    Asc,
    Desc
}
