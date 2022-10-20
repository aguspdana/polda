use super::FilterPredicate;

#[derive(Debug, Clone)]
pub struct Filter {
    pub column: String,
    pub predicate: FilterPredicate
}
