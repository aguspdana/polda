use crate::error::Error;
use crate::operation::{Operation, rebase};
use crate::transformable::Transformable;

pub struct Ot<T: Transformable> {
    doc: T,
    operations: Vec<Operation>,
    deleted: usize
}

impl<T: Transformable> Ot<T> {
    pub fn apply(
        &mut self,
        mut transactions: Vec<Vec<Operation>>,
        version: usize
    ) -> Result<Vec<Operation>, Error> {
        // TODO: Delete operations when the length reach the limit.
        if !self.syncable(version) {
            return Err(Error::Unsyncable);
        }

        if version != self.version() {
            let start = version - self.deleted;
            transactions = rebase(transactions, &self.operations[start..])?;
        }

        let mut undos: Vec<Operation> = vec![];
        let mut operations: Vec<Operation> = vec![];

        for tr in transactions.into_iter() {
            let mut applied = 0_usize;
            for op in tr.into_iter() {
                match self.doc.try_apply(op.clone()) {
                    Ok(Some(undo)) => {
                        undos.push(undo);
                        operations.push(op.clone());
                        applied += 1;
                    }
                    Ok(None) => {
                        for _ in 0..applied {
                            operations.pop();
                            let undo = undos.pop().unwrap();
                            self.doc.try_apply(undo).unwrap();
                        }
                    }
                    Err(e) => {
                        for _ in 0..applied {
                            operations.pop();
                            let undo = undos.pop().unwrap();
                            self.doc.try_apply(undo).unwrap();
                        }
                        return Err(e);
                    }
                }
            }
        }

        Ok(undos)
    }

    pub fn syncable(&self, version: usize) -> bool {
        if version > self.operations.len() + self.deleted {
            return false;
        }
        if version < self.deleted {
            return false;
        }
        true
    }

    pub fn doc(&self) -> &T {
        &self.doc
    }

    pub fn new(doc: T) -> Ot<T> {
        Ot {
            doc,
            operations: vec![],
            deleted: 0
        }
    }

    pub fn operations(&self, version: usize) -> Option<&[Operation]> {
        if self.syncable(version) {
            return None;
        }
        let start = version - self.deleted;
        Some(&self.operations[start..])
    }

    pub fn version(&self) -> usize {
        self.deleted + self.operations.len()
    }
}
