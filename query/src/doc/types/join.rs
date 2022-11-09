use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinColumn {
    pub left: String,
    pub right: String
}

