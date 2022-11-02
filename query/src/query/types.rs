use std::collections::HashMap;

use crate::data_type::DataType;

#[derive(Debug, Clone)]
pub struct SqlQuery {
    pub id: String,
    pub query: String
}

pub type Schema = HashMap<String, DataType>;
