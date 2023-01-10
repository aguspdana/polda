use serde::Deserialize;
use serde::Serialize;

use super::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "param")]
#[serde(rename_all = "snake_case")]
pub enum ComputeOperation {
    Add(Value),
    Subtract(Value),
    Multiply(Value),
    Divide(Value),
    IsEqualTo(Value),
    IsNotEqualTo(Value),
    IsLessThan(Value),
    IsLessThanEqual(Value),
    IsGreaterThan(Value),
    IsGreaterThanEqual(Value),
    IsNull,
    IsNotNull,
    And(Value),
    Or(Value),
    Xor(Value),
    Mean,
    Median,
    Min,
    Max,
}
