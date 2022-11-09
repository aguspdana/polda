use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectColumn {
    pub column: String,
    pub alias: String
}
