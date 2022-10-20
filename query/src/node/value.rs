#[derive(Debug, Clone)]
pub enum Value {
    Column(String),
    Constant(String)
}
