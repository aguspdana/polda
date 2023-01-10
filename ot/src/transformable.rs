use crate::Branch;
use crate::Error;
use crate::Operation;
use crate::Value;

pub trait Transformable {
    fn try_apply(&mut self, op: Operation) -> Result<Option<Operation>, Error>;
}

impl<T: Transformable + From<Value>> Transformable for Vec<T> {
    fn try_apply(&mut self, op: Operation) -> Result<Option<Operation>, Error> {
        use Operation::*;

        match op {
            Insert(mut path, vals) => {
                if path.len() == 0 {
                    Err(Error::InvalidOperation(Insert(path, vals)))
                } else if path.len() == 1 {
                    if let Branch::Index(i) = *path.branch(0) {
                        if i < self.len() {
                            return Err(Error::InvalidOperation(Insert(path, vals)));
                        }
                        let undo = Delete(path, vals.len());
                        let vals = vals.into_iter().map(|v| v.into());
                        self.splice(i..i, vals);
                        Ok(Some(undo))
                    } else {
                        Err(Error::InvalidOperation(Insert(path, vals)))
                    }
                } else {
                    if let Branch::Index(i) = *path.branch(0) {
                        if i < self.len() {
                            return Err(Error::InvalidOperation(Insert(path, vals)));
                        }
                        if let Some(undo) = self[i].try_apply(Insert(path, vals))? {
                            todo!()
                        }
                        todo!()
                    } else {
                        Err(Error::InvalidOperation(Insert(path, vals)))
                    }
                }
            }

            Delete(path, len) => {
                todo!()
            }

            InsertChars(path, chars) => {
                todo!()
            }

            DeleteChars(path, len) => {
                todo!()
            }

            Move(from, to) => {
                todo!()
            }

            Set(path, val) => {
                todo!()
            }

            Increment(path, by) => {
                todo!()
            }

            Decrement(path, by) => {
                todo!()
            }
        }
    }
}
