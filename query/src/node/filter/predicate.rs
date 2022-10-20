use crate::node::Value;

#[derive(Debug, Clone)]
pub enum FilterPredicate {
    IsEqualTo(Value),
    IsNotEqualTo(Value),
    IsLessThan(Value),
    IsGreaterThan(Value),
    IsIn(Vec<String>),
    IsNull,
    IsNotNull
}
