use super::Value;

#[derive(Debug, Clone)]
pub struct Filter {
    pub column: String,
    pub predicate: FilterPredicate
}

#[derive(Debug, Clone)]
pub enum FilterPredicate {
    IsEqualTo(Value),
    IsNotEqualTo(Value),
    IsLessThan(Value),
    IsLessThanEqual(Value),
    IsGreaterThan(Value),
    IsGreaterThanEqual(Value),
    IsNull,
    IsNotNull
}
