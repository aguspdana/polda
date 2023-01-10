use serde::Deserialize;
use serde::Serialize;

pub mod aggregate;
pub mod case;
pub mod compute;
pub mod filter;
pub mod join;
pub mod select;
pub mod sort;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct InputPort {
    pub id: String,
    pub name: InputName
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputName {
    Primary,
    Secondary
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "snake_case")]
pub enum Value {
    Column(String),
    Constant(String)
}
