use crate::error::Error;
use crate::operation::Operation;

pub trait Transformable {
    fn try_apply(&mut self, op: Operation) -> Result<Option<Operation>, Error>;
}
