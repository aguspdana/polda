use crate::operation::Operation;
use crate::position::Position;

#[derive(Debug, Clone)]
pub enum Error {
    IncompatiblePositions(Position, Position),
    InvalidOperation(Operation),
    NonexistentPosition(Position, Operation),
    Unsyncable
}
