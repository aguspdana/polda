use serde::Deserialize;
use serde::Serialize;

use super::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub column: String,
    pub predicate: FilterPredicate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "comparator")]
#[serde(rename_all = "snake_case")]
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
