use crate::data_type::DataType;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType
}
