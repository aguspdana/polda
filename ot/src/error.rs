use crate::Operation;
use crate::Path;

#[derive(Debug, Clone)]
pub enum Error {
    IncompatiblePositions(Path, Path),
    InvalidOperation(Operation),
    NonexistentPosition(Path, Operation),
    Unsyncable
}
