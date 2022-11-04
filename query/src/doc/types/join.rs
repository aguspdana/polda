#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross
}

#[derive(Debug, Clone)]
pub struct JoinColumn {
    pub left: String,
    pub right: String
}

