use serde::Deserialize;
use serde::Serialize;

use super::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub column: String,
    pub value: Value
}
