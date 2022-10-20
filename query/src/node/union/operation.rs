use crate::node::Position;

#[derive(Debug, Clone)]
pub enum UnionNodeOperation {
    SetPosition {
        position: Position
    },
    SetFirstInput {
        input: Option<String>
    },
    SetSecondInput {
        input: Option<String>
    }
}

impl UnionNodeOperation {
    pub fn map(self, _mapper: &UnionNodeOperation) -> UnionNodeOperation {
        self
    }

    pub fn transform_backward(self, _preceded_by: &UnionNodeOperation) -> Option<UnionNodeOperation> {
        Some(self)
    }

    pub fn transform_forward(self, _preceded_by: &UnionNodeOperation) -> Option<UnionNodeOperation> {
        Some(self)
    }
}
