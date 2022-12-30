use crate::error::Error;
use crate::position::BackTransform;
use crate::position::Branch;
use crate::position::Position;
use crate::position::PositionType;
use crate::value::Value;

/// {
///     "transactions": [
///         [
///             {
///                 "set": [["nodes", "xyz"], {...}]
///             },
///             {
///                 "insert": [["index", 0], ["xyz"]]
///             }
///         ]
///     ]
/// }

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Insert(Position, Vec<Value>),
    Delete(Position, usize),
    InsertChars(Position, String),
    DeleteChars(Position, usize),
    Move(Position, Position),
    Set(Position, Value),
    Increment(Position, f64),
    Decrement(Position, f64),
}

pub fn rebase(
    transactions: Vec<Vec<Operation>>,
    base: &[Operation]
) -> Result<Vec<Vec<Operation>>, Error> {
    let mut rebased: Vec<Option<Vec<Operation>>> = vec![];

    for (i, transaction) in transactions.clone().into_iter().enumerate() {
        let len = transaction.len();
        let mut rebased_transaction: Vec<Operation> = vec![];

        for (j, operation) in transaction.into_iter().enumerate() {
            use Branch::Index;
            use Operation::*;

            match operation {
                Insert(mut pos, val) => {
                    if !rebase_position(
                        &mut pos,
                        PositionType::Anchor,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }
                    rebased_transaction.push(Insert(pos, val));
                }

                Delete(pos, len) => {
                    let mut start = pos;
                    let mut end = start.clone();

                    if !end.is_root() {
                        if let Index(i) = end.leaf() {
                            let leaf = Index(*i + len);
                            end.set_branch(end.len() - 1, leaf);
                        }
                    }

                    if !rebase_position(
                        &mut start,
                        PositionType::RangeStart,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }

                    if !rebase_position(
                        &mut end,
                        PositionType::RangeEnd,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }

                    let len = if let (Index(s), Index(e)) = (start.leaf(), end.leaf()) {
                        if e > s {
                            e - s
                        } else {
                            0
                        }
                    } else {
                        0
                    };

                    if len == 0 {
                        break;
                    }

                    let op = Delete(start, len);
                    rebased_transaction.push(op);
                }

                InsertChars(mut pos, chars) => {
                    if !rebase_position(
                        &mut pos,
                        PositionType::Anchor,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }
                    rebased_transaction.push(InsertChars(pos, chars));
                }

                DeleteChars(pos, len) => {
                    if !pos.is_leaf_index() {
                        return Err(Error::InvalidOperation(Delete(pos, len)));
                    }

                    let mut start = pos;
                    let mut end = start.clone();

                    if !end.is_root() {
                        if let Index(i) = end.leaf() {
                            let leaf = Index(*i + len);
                            end.set_branch(end.len() - 1, leaf);
                        }
                    }

                    if !rebase_position(
                        &mut start,
                        PositionType::RangeStart,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }

                    if !rebase_position(
                        &mut end,
                        PositionType::RangeEnd,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }

                    let len = if let (Index(s), Index(e)) = (start.leaf(), end.leaf()) {
                        if e > s {
                            e - s
                        } else {
                            0
                        }
                    } else {
                        0
                    };

                    if len == 0 {
                        break;
                    }

                    let op = DeleteChars(start, len);
                    rebased_transaction.push(op);
                }

                Move(mut from, mut to) => {
                    if !rebase_position(
                        &mut from,
                        PositionType::Exact,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }
                    if !rebase_position(
                        &mut to,
                        PositionType::Anchor,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }
                    rebased_transaction.push(Move(from, to));
                }

                Set(mut pos, val) => {
                    if !rebase_position(
                        &mut pos,
                        PositionType::Exact,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }
                    rebased_transaction.push(Set(pos, val));
                }

                Increment(mut pos, by) => {
                    if !rebase_position(
                        &mut pos,
                        PositionType::Change,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }
                    rebased_transaction.push(Increment(pos, by));
                }

                Decrement(mut pos, by) => {
                    if !rebase_position(
                        &mut pos,
                        PositionType::Change,
                        base,
                        &transactions[i][..j],
                        &*rebased_transaction,
                        &transactions[..i],
                        &*rebased
                    )? {
                        break;
                    }
                    rebased_transaction.push(Decrement(pos, by));
                }
            }
        }

        if rebased_transaction.len() == len {
            rebased.push(Some(rebased_transaction));
        } else {
            rebased.push(None);
        }
    }

    let rebased = rebased
        .into_iter()
        .filter(|tr| tr.is_some())
        .map(|tr| tr.unwrap())
        .collect();

    Ok(rebased)
}

fn rebase_position(
    position: &mut Position,
    position_type: PositionType,
    base: &[Operation],
    prev_ops: &[Operation],
    rebased_prev_ops: &[Operation],
    prev_transactions: &[Vec<Operation>],
    rebased_prev_transactions: &[Option<Vec<Operation>>]
) -> Result<bool, Error> {
    for (i, op) in prev_ops.iter().enumerate().rev() {
        match position.transform_backward_or_map(op, Some(&rebased_prev_ops[i]), position_type)? {
            BackTransform::Transformed => {}
            BackTransform::Mapped => {
                if transform_forward_many(position, position_type, &rebased_prev_ops[i+1..], true)? {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            BackTransform::None => return Ok(false)
        }
    }

    for (i, tr) in prev_transactions.iter().enumerate().rev() {
        for (j, op) in tr.iter().enumerate().rev() {
            if let Some(tr) = &rebased_prev_transactions[i] {
                match position.transform_backward_or_map(op, Some(&tr[j]), position_type)? {
                    BackTransform::Transformed => {}
                    BackTransform::Mapped => {
                        if !transform_forward_many(position, position_type, &tr[j+1..], true)? {
                            return Ok(false);
                        }
                        for tr in rebased_prev_transactions[i+1..].iter() {
                            if let Some(tr) = tr {
                                if !transform_forward_many(position, position_type, &*tr, true)? {
                                    return Ok(false);
                                }
                            }
                        }
                        if !transform_forward_many(position, position_type, &*rebased_prev_ops, true)? {
                            return Ok(false);
                        }
                        return Ok(true);
                    }
                    BackTransform::None => return Ok(false)
                }
            }
        }
    }

    if !transform_forward_many(position, position_type, base, false)? {
        return Ok(false);
    }
    for tr in rebased_prev_transactions.iter() {
        if let Some(tr) = tr {
            if !transform_forward_many(position, position_type, &*tr, true)? {
                return Ok(false);
            }
        }
    }
    transform_forward_many(position, position_type, rebased_prev_ops, true)
}

fn transform_forward_many(
    position: &mut Position,
    position_type: PositionType,
    operations: &[Operation],
    extend_range: bool
) -> Result<bool, Error> {
    for op in operations.iter() {
        if !position.transform_forward(op, position_type, extend_range)? {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use crate::operation::Operation;
    use crate::position::Position;
    use crate::position::Branch::*;
    use crate::value::Value;
    use super::Operation::*;
    use super::rebase;

    macro_rules! expect_rebase_unchanged {
        ($($transaction:expr),+) => {
            let transactions = vec![vec![$($transaction),+]];
            let base = [];
            let transformed = rebase(transactions.clone(), &base).unwrap();
            assert_eq!(transformed, transactions);
        };
    }

    macro_rules! expect_rebase_eq {
        ($op:expr, $base:expr, $expect:expr) => {
            let transactions = vec![vec![$op]];
            let base = [$base];
            let transformed = rebase(transactions.clone(), &base).unwrap();
            assert_eq!(transformed, vec![vec![$expect]]);
        };
    }

    macro_rules! expect_rebase_to_none {
        ($op:expr, $base:expr) => {
            let transactions = vec![vec![$op]];
            let base = [$base];
            let expect: Vec<Vec<Operation>> = vec![];
            assert_eq!(rebase(transactions.clone(), &base).unwrap(), expect);
        };
    }

    // Rebase Insert ...

    #[test]
    fn rebase_insert_insert_1() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_insert_insert_2() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_insert_insert_3() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_insert_delete_1() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                2
            )
        ];
    }

    #[test]
    fn rebase_insert_delete_2() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                2
            )
        ];
    }

    #[test]
    fn rebase_insert_delete_3() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                2
            )
        ];
    }

    #[test]
    fn rebase_insert_insert_chars_1() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_insert_insert_chars_2() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_insert_insert_chars_3() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(3),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_insert_delete_chars_1() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_insert_delete_chars_2() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_insert_delete_chars_3() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(3),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_insert_move_1() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        ];
    }

    #[test]
    fn rebase_insert_move_2() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ])
            )
        ];
    }

    #[test]
    fn rebase_insert_move_3() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ])
            )
        ];
    }

    #[test]
    fn rebase_insert_move_4() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ])
            )
        ];
    }

    #[test]
    fn rebase_insert_move_5() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ])
            )
        ];
    }

    #[test]
    fn rebase_insert_move_6() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ])
            )
        ];
    }

    #[test]
    fn rebase_insert_set_1() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_insert_set_2() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_insert_set_3() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_insert_increment_1() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_insert_increment_2() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_insert_increment_3() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(3),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_insert_decrement_1() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_insert_decrement_2() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_insert_decrement_3() {
        expect_rebase_unchanged![
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(3),
                ]),
                1.0
            )
        ];
    }

    // Rebase Delete ...

    #[test]
    fn rebase_delete_insert_1() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_delete_insert_2() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(7)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_delete_insert_3() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_delete_delete_1() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                2
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                2
            )
        ];
    }

    #[test]
    fn rebase_delete_delete_2() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                3
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_delete_delete_3() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_delete_insert_chars_1() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_delete_insert_chars_2() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(8),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_delete_insert_chars_3() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_delete_delete_chars_1() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_delete_delete_chars_2() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(8),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_delete_delete_chars_3() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_delete_move_1() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        ];
    }

    #[test]
    fn rebase_delete_move_2() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(8)
                ])
            )
        ];
    }

    #[test]
    fn rebase_delete_move_3() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ])
            )
        ];
    }

    #[test]
    fn rebase_delete_move_4() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ])
            )
        ];
    }

    #[test]
    fn rebase_delete_move_5() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(8)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ])
            )
        ];
    }

    #[test]
    fn rebase_delete_move_6() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ])
            )
        ];
    }

    #[test]
    fn rebase_delete_set_1() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_delete_set_2() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(8),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_delete_set_3() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_delete_increment_1() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_delete_increment_2() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(8),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_delete_increment_3() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_delete_decrement_1() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_delete_decrement_2() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(8),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_delete_decrement_3() {
        expect_rebase_unchanged![
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                3
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                ]),
                1.0
            )
        ];
    }

    // Rebase Move ...

    #[test]
    fn rebase_move_insert_1() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_move_insert_2() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_move_insert_3() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_move_insert_4() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_move_insert_5() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_move_insert_6() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                vec![Value::Null]
            )
        ];
    }

    #[test]
    fn rebase_move_delete_1() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                2
            )
        ];
    }

    #[test]
    fn rebase_move_delete_2() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_move_delete_3() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_move_insert_chars_1() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_move_insert_chars_2() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_move_insert_chars_3() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_move_insert_chars_4() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_move_insert_chars_5() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                    Index(0)
                ]),
                String::from("abc")
            )
        ];
    }

    #[test]
    fn rebase_move_delete_chars_1() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_move_delete_chars_2() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_move_delete_chars_3() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                    Index(0)
                ]),
                3
            )
        ];
    }

    #[test]
    fn rebase_move_move_1() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ])
            )
        ];
    }

    #[test]
    fn rebase_move_move_2() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        ];
    }

    #[test]
    fn rebase_move_move_3() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ])
            )
        ];
    }

    #[test]
    fn rebase_move_move_4() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                ])
            )
        ];
    }

    #[test]
    fn rebase_move_move_5() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                ])
            )
        ];
    }

    #[test]
    fn rebase_move_move_6() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ])
            )
        ];
    }

    #[test]
    fn rebase_move_set_1() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_move_set_2() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_move_set_3() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_move_set_4() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_move_set_5() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                ]),
                Value::Number(1.0)
            )
        ];
    }

    #[test]
    fn rebase_move_increment_1() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_increment_2() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_increment_3() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_increment_4() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_increment_5() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_decrement_1() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_decrement_2() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_decrement_3() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_decrement_4() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                ]),
                1.0
            )
        ];
    }

    #[test]
    fn rebase_move_decrement_5() {
        expect_rebase_unchanged![
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(5),
                ]),
                1.0
            )
        ];
    }

    // Rebase Insert ON ...

    #[test]
    fn rebase_insert_on_insert_1() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("abc"))]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_insert_2() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("abc"))]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_insert_3() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("abc"))]
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_delete_1() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("xyz"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_delete_2() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                3
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("xyz"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_delete_3() {
        expect_rebase_to_none!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::String(String::from("xyz"))]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                3
            )
        );
    }

    #[test]
    fn rebase_insert_on_insert_chars_1() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(3)
                ]),
                String::from("abc")
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_delete_chars_1() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(3)
                ]),
                3
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_move_1() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_move_2() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_move_3() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_move_4() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_move_5() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_move_6() {
        expect_rebase_to_none!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Field(String::from("x")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Field(String::from("y"))
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Field(String::from("x"))
                ]),
            )
        );
    }

    #[test]
    fn rebase_insert_on_move_7() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Field(String::from("x")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Field(String::from("x"))
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Field(String::from("y"))
                ]),
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Field(String::from("y")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_set_1() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Value::String(String::from("ooo"))
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("def"))]
            )
        );
    }

    #[test]
    fn rebase_insert_on_increment_1() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::Number(1.0)]
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                1.0
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::Number(1.0)]
            )
        );
    }

    #[test]
    fn rebase_insert_on_decrement_1() {
        expect_rebase_eq!(
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::Number(1.0)]
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                1.0
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::Number(1.0)]
            )
        );
    }

    // Rebase Delete ON ...

    #[test]
    fn rebase_delete_on_insert_1() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("abc"))]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            )
        );
    }

    #[test]
    fn rebase_delete_on_insert_2() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                1
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                vec![Value::String(String::from("abc"))]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1
            )
        );
    }

    #[test]
    fn rebase_delete_on_insert_3() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                3
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::String(String::from("abc"))]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                4
            )
        );
    }

    #[test]
    fn rebase_delete_on_insert_4() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                3
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::String(String::from("abc"))]
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                3
            )
        );
    }

    #[test]
    fn rebase_delete_on_delete_1() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                3
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                3
            )
        );
    }

    #[test]
    fn rebase_delete_on_delete_2() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                2
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            )
        );
    }

    #[test]
    fn rebase_delete_on_delete_3() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                4
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            )
        );
    }

    #[test]
    fn rebase_delete_on_delete_4() {
        expect_rebase_to_none!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                    Index(8)
                ]),
                4
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            )
        );
    }

    #[test]
    fn rebase_delete_on_insert_chars_1() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(3),
                    Index(3)
                ]),
                String::from("abc")
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            )
        );
    }

    #[test]
    fn rebase_delete_on_delete_chars_1() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(3),
                    Index(3)
                ]),
                3
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            )
        );
    }

    #[test]
    fn rebase_delete_on_move_1() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(6)
                ]),
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                4
            )
        );
    }

    #[test]
    fn rebase_delete_on_move_2() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(7)
                ]),
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                3
            )
        );
    }

    #[test]
    fn rebase_delete_on_move_3() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(6)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                5
            )
        );
    }

    #[test]
    fn rebase_delete_on_move_4() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            )
        );
    }

    #[test]
    fn rebase_delete_on_move_5() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(8)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(6)
                ]),
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            )
        );
    }

    #[test]
    fn rebase_delete_on_set_1() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                Value::String(String::from("ooo"))
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            )
        );
    }

    #[test]
    fn rebase_delete_on_increment_1() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                1.0
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            )
        );
    }

    #[test]
    fn rebase_delete_on_decrement_1() {
        expect_rebase_eq!(
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                1.0
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                4
            )
        );
    }

    // Rebase Move ON [...]

    #[test]
    fn rebase_move_on_insert_1() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::Number(1.0)]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_insert_2() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                vec![Value::Number(1.0)]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_insert_3() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_insert_4() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ])
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                vec![Value::Number(1.0)]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_insert_5() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ])
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::Number(1.0)]
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_delete_1() {
        expect_rebase_to_none!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            )
        );
    }

    #[test]
    fn rebase_move_on_delete_2() {
        expect_rebase_to_none!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ])
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            )
        );
    }

    #[test]
    fn rebase_move_on_delete_3() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_insert_chars_1() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(5)
                ]),
                String::from("abc")
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_delete_chars_1() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(5)
                ]),
                3
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_insert_chars_2() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Index(5)
                ]),
                String::from("abc")
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_delete_chars_2() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Index(5)
                ]),
                3
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_move_1() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_move_2() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_move_3() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("b")),
                    Field(String::from("c"))
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_move_4() {
        expect_rebase_eq!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("b")),
                    Field(String::from("c"))
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(5)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            )
        );
    }

    #[test]
    fn rebase_move_on_move_5() {
        expect_rebase_to_none!(
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Move(
                Position(vec![
                    Field(String::from("b"))
                ]),
                Position(vec![
                    Field(String::from("a"))
                ])
            )
        );
    }

    #[test]
    fn rebase_set_on_insert_1() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::Number(2.0)
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::Number(1.0)]
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                Value::Number(2.0)
            )
        );
    }

    #[test]
    fn rebase_set_on_insert_2() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::Number(2.0)
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                vec![Value::Number(1.0)]
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::Number(2.0)
            )
        );
    }

    #[test]
    fn rebase_set_on_insert_3() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                    Field(String::from("b"))
                ]),
                Value::Number(2.0)
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::Number(1.0)]
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(3),
                    Field(String::from("b"))
                ]),
                Value::Number(2.0)
            )
        );
    }

    #[test]
    fn rebase_set_on_delete_1() {
        expect_rebase_to_none!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                    Field(String::from("b"))
                ]),
                Value::Number(2.0)
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            )
        );
    }

    #[test]
    fn rebase_set_on_delete_2() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(4),
                    Field(String::from("b"))
                ]),
                Value::Number(2.0)
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                    Field(String::from("b"))
                ]),
                Value::Number(2.0)
            )
        );
    }

    #[test]
    fn rebase_set_on_insert_chars_1() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            ),
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                    Index(0)
                ]),
                String::from("abc")
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            )
        );
    }

    #[test]
    fn rebase_set_on_delete_chars_1() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            ),
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(2),
                    Index(0)
                ]),
                4
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            )
        );
    }

    #[test]
    fn rebase_set_on_move_1() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                Value::String(String::from("def"))
            )
        );
    }

    #[test]
    fn rebase_set_on_move_2() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("b")),
                    Index(4)
                ]),
            ),
            Set(
                Position(vec![
                    Field(String::from("b")),
                    Index(4)
                ]),
                Value::String(String::from("def"))
            )
        );
    }

    #[test]
    fn rebase_set_on_move_3() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            ),
            Move(
                Position(vec![
                    Field(String::from("b")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                Value::String(String::from("def"))
            )
        );
    }

    #[test]
    fn rebase_set_on_move_4() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![]),
            ),
            Set(
                Position(vec![]),
                Value::String(String::from("def"))
            )
        );
    }

    #[test]
    fn rebase_set_on_set_1() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("abc"))
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            )
        );
    }

    #[test]
    fn rebase_set_on_set_2() {
        expect_rebase_to_none!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("def"))
            ),
            Set(
                Position(vec![
                    Field(String::from("a"))
                ]),
                Value::String(String::from("abc"))
            )
        );
    }

    #[test]
    fn rebase_set_on_increment_1() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::Number(1.0)
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::Number(1.0)
            )
        );
    }

    #[test]
    fn rebase_set_on_increment_2() {
        expect_rebase_eq!(
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::Number(1.0)
            ),
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::Number(1.0)
            )
        );
    }

    #[test]
    fn rebase_increment_on_insert_1() {
        expect_rebase_eq!(
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::Number(1.0)]
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                1.0
            )
        );
    }

    #[test]
    fn rebase_increment_on_insert_2() {
        expect_rebase_eq!(
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                1.0
            ),
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                vec![Value::Number(1.0)]
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(1)
                ]),
                1.0
            )
        );
    }

    #[test]
    fn rebase_increment_on_delete_1() {
        expect_rebase_to_none!(
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            ),
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                2
            )
        );
    }

    #[test]
    fn rebase_increment_on_move_1() {
        expect_rebase_eq!(
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(3)
                ]),
                1.0
            )
        );
    }

    #[test]
    fn rebase_increment_on_move_2() {
        expect_rebase_eq!(
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                1.0
            ),
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ])
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                1.0
            )
        );
    }

    #[test]
    fn rebase_increment_on_set_1() {
        expect_rebase_to_none!(
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            ),
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::Number(2.0)
            )
        );
    }

    #[test]
    fn rebase_increment_on_increment_1() {
        expect_rebase_eq!(
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                1.0
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                1.0
            ),
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
                1.0
            )
        );
    }

    #[test]
    fn rebase_delete_on_insert() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    2
                )
            ]
        ];
        let base = [
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("ooo"))]
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(1)
                    ]),
                    2
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_on_delete() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    2
                )
            ]
        ];
        let base = [
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_on_insert_chars() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(2)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(3)
                ]),
                String::from("ggg")
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(
            rebased,
            transactions
        );
    }

    #[test]
    fn rebase_delete_on_delete_chars() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(2)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(3)
                ]),
                3
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_delete_on_move() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    5
                )
            ]
        ];
        let base = [
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    5
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_on_set() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                Value::String(String::from("ooo"))
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_delete_on_increment() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_delete_on_decrement() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_insert_chars_on_insert() {
        let transactions = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        let base = [
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("ooo"))]
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(1),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_insert_chars_on_delete() {
        let transactions = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        let base = [
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_insert_chars_on_insert_chars() {
        let transactions = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        let base = [
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(0)
                ]),
                String::from("ggg")
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        let expect = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(3)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_insert_chars_on_delete_chars() {
        let transactions = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(3)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        let base = [
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(0)
                ]),
                3
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        let expect = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_insert_chars_on_move() {
        let transactions = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        let base = [
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(3),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_insert_chars_on_set() {
        let transactions = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        let base = [
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Value::String(String::from("ooo"))
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_insert_chars_on_increment() {
        let transactions = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        let base = [
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_insert_chars_on_decrement() {
        let transactions = vec![
            vec![
                InsertChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    String::from("xyz")
                )
            ]
        ];
        let base = [
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_delete_chars_on_insert() {
        let transactions = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    2
                )
            ]
        ];
        let base = [
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("ooo"))]
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(1),
                        Index(0)
                    ]),
                    2
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_chars_on_delete() {
        let transactions = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    2
                )
            ]
        ];
        let base = [
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_chars_on_insert_chars() {
        let transactions = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(0)
                ]),
                String::from("ggg")
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        let expect = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(3)
                    ]),
                    4
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_chars_on_delete_chars() {
        let transactions = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(3)
                ]),
                3
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        let expect = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    3
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_chars_on_move() {
        let transactions = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    5
                )
            ]
        ];
        let base = [
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(3),
                        Index(0)
                    ]),
                    5
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_chars_on_set() {
        let transactions = vec![
            vec![
                Delete(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Value::String(String::from("ooo"))
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_delete_chars_on_increment() {
        let transactions = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_delete_chars_on_decrement() {
        let transactions = vec![
            vec![
                DeleteChars(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0),
                        Index(0)
                    ]),
                    4
                )
            ]
        ];
        let base = [
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(2)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_set_on_insert() {
        let transactions = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    Value::String(String::from("xyz"))
                )
            ]
        ];
        let base = [
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("ooo"))]
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(1)
                    ]),
                    Value::String(String::from("xyz"))
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_set_on_delete() {
        let transactions = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    Value::String(String::from("xyz"))
                )
            ]
        ];
        let base = [
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_set_on_insert_chars() {
        let transactions = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    Value::String(String::from("xyz"))
                )
            ]
        ];
        let base = [
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(3)
                ]),
                String::from("ggg")
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_set_on_delete_chars() {
        let transactions = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    Value::String(String::from("xyz"))
                )
            ]
        ];
        let base = [
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(0),
                    Index(3)
                ]),
                3
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_set_on_move() {
        let transactions = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    Value::String(String::from("xyz"))
                )
            ]
        ];
        let base = [
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(3)
                    ]),
                    Value::String(String::from("xyz"))
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_set_on_set() {
        let transactions = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    Value::String(String::from("xyz"))
                )
            ]
        ];
        let base = [
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Value::String(String::from("ooo"))
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_set_on_increment() {
        let transactions = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    Value::Number(1.0)
                )
            ]
        ];
        let base = [
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_set_on_decrement() {
        let transactions = vec![
            vec![
                Set(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    Value::Number(1.0)
                )
            ]
        ];
        let base = [
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_increment_on_insert() {
        let transactions = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("ooo"))]
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(1)
                    ]),
                    1.0
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_increment_on_delete() {
        let transactions = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_increment_on_insert_chars() {
        let transactions = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(3)
                ]),
                String::from("ggg")
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_increment_on_delete_chars() {
        let transactions = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(3)
                ]),
                3
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_increment_on_move() {
        let transactions = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(3)
                    ]),
                    1.0
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_increment_on_set() {
        let transactions = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Value::String(String::from("ooo"))
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_increment_on_increment() {
        let transactions = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_increment_on_decrement() {
        let transactions = vec![
            vec![
                Increment(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_decrement_on_insert() {
        let transactions = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Insert(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                vec![Value::String(String::from("ooo"))]
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(1)
                    ]),
                    1.0
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_decrement_on_delete() {
        let transactions = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Delete(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_decrement_on_insert_chars() {
        let transactions = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            InsertChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(3)
                ]),
                String::from("ggg")
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_decrement_on_delete_chars() {
        let transactions = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            DeleteChars(
                Position(vec![
                    Field(String::from("a")),
                    Index(1),
                    Index(3)
                ]),
                3
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_decrement_on_move() {
        let transactions = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Move(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Position(vec![
                    Field(String::from("a")),
                    Index(4)
                ]),
            )
        ];
        let rebased = rebase(transactions, &base).unwrap();
        let expect = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(3)
                    ]),
                    1.0
                )
            ]
        ];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_decrement_on_set() {
        let transactions = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Set(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                Value::String(String::from("ooo"))
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        let expect: Vec<Vec<Operation>> = vec![];
        assert_eq!(rebased, expect);
    }

    #[test]
    fn rebase_decrement_on_increment() {
        let transactions = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Increment(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }

    #[test]
    fn rebase_decrement_on_decrement() {
        let transactions = vec![
            vec![
                Decrement(
                    Position(vec![
                        Field(String::from("a")),
                        Index(0)
                    ]),
                    1.0
                )
            ]
        ];
        let base = [
            Decrement(
                Position(vec![
                    Field(String::from("a")),
                    Index(0)
                ]),
                1.0
            )
        ];
        let rebased = rebase(transactions.clone(), &base).unwrap();
        assert_eq!(rebased, transactions);
    }
}
