pub mod aggregate;
pub mod filter;
pub mod join;
pub mod select;
pub mod sort;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum InputName {
    Primary,
    Secondary
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct InputPort {
    pub id: String,
    pub name: InputName
}

#[derive(Debug, Clone)]
pub enum Value {
    Column(String),
    Constant(String)
}
