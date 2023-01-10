use ot::Value;
use serde::Deserialize;
use serde::Serialize;
use ot::Branch;
use ot::Error;
use ot::Operation;
use ot::Transformable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

impl Transformable for Position {
    fn try_apply(
        &mut self,
        op: Operation
    ) -> Result<Option<Operation>, Error> {
        use Operation::*;

        match op {
            Set(pos, val) => {
                if pos.len() == 1 {
                    if let (Branch::Field(f), Value::Number(v)) = (pos.branch(0), val) {
                        match &**f {
                            "x" => {
                                let undo = Set(pos, Value::Number(self.x));
                                self.x = v;
                                Ok(Some(undo))
                            }
                            "y" => {
                                let undo = Set(pos, Value::Number(self.x));
                                self.y = v;
                                Ok(Some(undo))
                            }
                            _ => Err(Error::InvalidOperation(Set(pos, val)))
                        }
                    } else {
                        Err(Error::InvalidOperation(Set(pos, val)))
                    }
                } else {
                    Err(Error::InvalidOperation(Set(pos, val)))
                }
            }

            op => Err(Error::InvalidOperation(op))
        }
    }
}
