use crate::node::Position;

#[derive(Debug, Clone)]
pub enum LoadCsvNodeOperation {
    SetPosition {
        position: Position
    },
    SetPath {
        path: String
    }
}

impl LoadCsvNodeOperation {
    pub fn map(self, mapper: &LoadCsvNodeOperation) -> LoadCsvNodeOperation {
        panic!("Can't map {:?} to {:?}", self, mapper);
    }

    pub fn transform_backward(self, _preceded_by: &LoadCsvNodeOperation) -> Option<LoadCsvNodeOperation> {
        Some(self)
    }

    pub fn transform_forward(self, _preceded_by: &LoadCsvNodeOperation) -> Option<LoadCsvNodeOperation> {
        Some(self)
    }
}
