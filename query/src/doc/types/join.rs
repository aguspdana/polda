#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full
}

#[derive(Debug, Clone)]
pub struct JoinColumn {
    pub left: String,
    pub right: String
}

