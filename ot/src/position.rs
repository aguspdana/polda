// TODO: This code looks like it's fresh out of Frankenstein's lab.
// Make it more readable!

use crate::{operation::Operation, error::Error};

#[derive(Clone, Debug, PartialEq)]
pub enum Branch {
    Field(String),
    Index(usize)
}

impl Branch {
    pub fn is_field(&self) -> bool {
        if let Branch::Field(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_index(&self) -> bool {
        if let Branch::Index(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionType {
    /// A position that can be deleted only if it's within the delete range,
    /// exclusive to the start and end position.
    /// I.e. `Insert`, `InsertChars`, and `Move` target position.
    Anchor,
    /// A position that can't be deleted.
    RangeStart,
    /// A position that can't be deleted.
    RangeEnd,
    /// A position that can be deleted and can be moved.
    /// I.e. `Set` and `Move` initial position.
    Exact,
    /// Like `Exact` but the position no longer exists after it's set.
    Change
}

pub enum BackTransform {
    Transformed,
    Mapped,
    None
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position(pub Vec<Branch>);

impl Position {
    /// # Panic
    /// Panic if `at` is out of range.
    pub fn branch(&self, at: usize) -> &Branch {
        &self.0[at]
    }

    /// Check if `self` is an anchetor of `of`.
    pub fn is_ancestor(&self, of: &Position) -> bool {
        if self.0.len() >= of.0.len() {
            return false;
        }
        self.0 == of.0[..self.len()]
    }

    /// Check if `self` is an anchetor of `of`.
    pub fn is_ancestor_or_equal(&self, of: &Position) -> bool {
        if self.0.len() > of.0.len() {
            return false;
        }
        self.0 == of.0[..self.len()]
    }

    /// Check if the positions have the same length, each branch has the same
    /// branch type, and each field branch has the same value.
    pub fn is_compatible(&self, other: &Position) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            use Branch::*;
            match (a, b) {
                (Field(a), Field(b)) => {
                    if a != b {
                        return false;
                    }
                }
                (Index(_), Index(_)) => {}
                _ => return false
            }
        }
        true
    }

    pub fn is_leaf_field(&self) -> bool {
        if self.len() == 0 {
            return false;
        }
        self.leaf().is_field()
    }

    pub fn is_leaf_index(&self) -> bool {
        if self.len() == 0 {
            return false;
        }
        self.leaf().is_index()
    }

    /// Check if the parent of `self` is an anchetor of `of`.
    pub fn is_parent_ancestor(&self, of: &Position) -> bool {
        if self.0.len() == 0 {
            return false;
        }
        if self.0.len() == 1 {
            return true;
        }
        if self.0.len() > of.0.len() {
            return false;
        }
        let end = self.0.len() - 1;
        self.0[..end] == of.0[..end]
    }

    pub fn is_root(&self) -> bool {
        self.0.len() == 0
    }

    pub fn is_sibling(&self, other: &Position) -> bool {
        if self.0.len() == 0 {
            return false;
        }
        if self.0.len() != other.0.len() {
            return false;
        }
        let end = self.0.len() - 1;
        self.0[..end] == other.0[..end]
    }

    /// Get the last branch.
    ///
    /// # Panic
    /// Panic if the position is root.
    pub fn leaf(&self) -> &Branch {
        &self.0[self.0.len() - 1]
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Mapping with non-mapper operations returns error.
    pub fn map(
        &mut self,
        before: &Operation,
        after: &Operation
    ) {
        use Branch::Index;
        use Operation::*;

        match (before, after) {
            (Insert(pos_before, values_before), Insert(pos_after, values_after)) => {
                if !pos_before.is_parent_ancestor(&self)
                    || values_before.len() != values_after.len()
                {
                    panic!("Invalid mapper");
                }

                if let Index(i) = *self.branch(pos_before.len() - 1) {
                    if let Index(i_b) = pos_before.leaf() {
                        if let Index(i_a) = pos_after.leaf() {
                            self.replace_head(pos_before.len(), pos_after.clone());
                            if i_b == i_a {
                                return ();
                            }
                            if i < *i_b {
                                panic!("Invalid mapper");
                            }
                            let offset = i - i_b;
                            let new_i = i_a + offset;
                            let branch = Index(new_i);
                            self.set_branch(pos_after.len() - 1, branch);
                        } else {
                            panic!("Invalid operation");
                        }
                    } else {
                        panic!("Invalid operation");
                    }
                } else {
                    panic!("Invalid mapper");
                }
            }

            (InsertChars(pos_before, chars_before), InsertChars(pos_after, chars_after)) => {
                if !pos_before.is_sibling(&self)
                    || chars_before.len() != chars_after.len() {
                    panic!("Invalid mapper");
                }

                if let Index(i) = *self.leaf() {
                    if let Index(i_b) = pos_before.leaf() {
                        if let Index(i_a) = pos_after.leaf() {
                            if i_b == i_a {
                                return ();
                            }
                            if i < *i_b {
                                panic!("Invalid mapper");
                            }
                            let offset = i - i_b;
                            let new_i = i_a + offset;
                            let branch = Index(new_i);
                            self.set_branch(pos_after.len() - 1, branch);
                        } else {
                            panic!("Invalid operation");
                        }
                    } else {
                        panic!("Invalid operation");
                    }
                } else {
                    panic!("Invalid mapper");
                }
            }

            (Set(pos_before, _), Set(pos_after, _)) => {
                if self.len() < pos_before.len()
                    || self.0[..pos_before.len()] == pos_before.0
                {
                    panic!("Invalid mapper");
                }
                self.replace_head(pos_before.len(), pos_after.clone());
            }

            (_, _) => panic!("Invalid mapper")
        }
    }

    fn replace_head(&mut self, at: usize, head: Position) {
        self.0.splice(..at, head.0);
    }

    /// Set the branch at the level.
    ///
    /// # Panic
    /// Panic if `at` is out of range.
    pub fn set_branch(&mut self, at: usize, branch: Branch) {
        self.0[at] = branch;
    }

    pub fn transform_backward_or_map(
        &mut self,
        op: &Operation,
        map: Option<&Operation>,
        position_type: PositionType
    ) -> Result<BackTransform, Error> {
        use Branch::Field;
        use Branch::Index;
        use Operation::*;

        match op {
            Insert(pos, values) => {
                if pos.is_root() {
                    return Err(Error::InvalidOperation(Insert(pos.clone(), values.clone())));
                }
                if !pos.is_parent_ancestor(&self) {
                    return Ok(BackTransform::Transformed);
                }
                let at = pos.len() - 1;
                if let (Index(op_i), Index(i)) = (pos.leaf(), self.branch(at)) {
                    if *i >= op_i + values.len() {
                        let branch = Index(i - values.len());
                        self.set_branch(at, branch);
                        Ok(BackTransform::Transformed)
                    } else if i >= op_i {
                        if let Some(Insert(map_pos, _)) = map {
                            if let Index(map_i) = map_pos.leaf() {
                                let offset = i - op_i;
                                let new_i = map_i + offset;
                                let branch = Index(new_i);
                                self.replace_head(pos.len(), map_pos.clone());
                                self.set_branch(map_pos.len() - 1, branch);
                                Ok(BackTransform::Mapped)
                            } else {
                                Ok(BackTransform::None)
                            }
                        } else if self.len() == pos.len() && (
                            position_type == PositionType::Anchor
                            || position_type == PositionType::RangeStart
                            || position_type == PositionType::RangeEnd
                        ) {
                            *self = pos.clone();
                            Ok(BackTransform::Transformed)
                        } else {
                            Ok(BackTransform::None)
                        }
                    } else {
                        Ok(BackTransform::Transformed)
                    }
                } else {
                    Err(Error::IncompatiblePositions(pos.clone(), self.clone()))
                }
            }

            Delete(pos, len) => {
                if pos.is_root() {
                    return Err(Error::InvalidOperation(Delete(pos.clone(), *len)));
                }
                if !pos.is_parent_ancestor(&self) {
                    return Ok(BackTransform::Transformed);
                }
                let at = pos.len() - 1;
                if let (Index(op_i), Index(i)) = (pos.leaf(), self.branch(at)) {
                    if i >= op_i {
                        let branch = Index(i + len);
                        self.set_branch(at, branch);
                    }
                    return Ok(BackTransform::Transformed);
                }
                Err(Error::IncompatiblePositions(pos.clone(), self.clone()))
            }

            InsertChars(pos, chars) => {
                if !pos.is_sibling(&self) {
                    return Ok(BackTransform::Transformed);
                }
                if let (Index(op_i), Index(i)) = (pos.leaf(), self.leaf()) {
                    if *i >= op_i + chars.len() {
                        let branch = Index(i - chars.len());
                        self.set_branch(self.len() - 1, branch);
                        Ok(BackTransform::Transformed)
                    } else if i >= op_i {
                        if let Some(Insert(map_pos, _)) = map {
                            if let Index(map_i) = map_pos.leaf() {
                                let offset = i - op_i;
                                let new_i = map_i + offset;
                                let branch = Index(new_i);
                                self.replace_head(pos.len(), map_pos.clone());
                                self.set_branch(map_pos.len() - 1, branch);
                                Ok(BackTransform::Mapped)
                            } else {
                                Ok(BackTransform::None)
                            }
                        } else if self.len() == pos.len() && (
                            position_type == PositionType::Anchor
                            || position_type == PositionType::RangeStart
                            || position_type == PositionType::RangeEnd
                        ) {
                            *self = pos.clone();
                            Ok(BackTransform::Transformed)
                        } else {
                            Ok(BackTransform::None)
                        }
                    } else {
                        Ok(BackTransform::Transformed)
                    }
                } else {
                    Err(Error::IncompatiblePositions(self.clone(), pos.clone()))
                }
            }

            DeleteChars(pos, len) => {
                if !pos.is_sibling(&self) {
                    return Ok(BackTransform::Transformed);
                }
                let at = pos.len() - 1;
                if let (Index(op_i), Index(i)) = (pos.leaf(), self.branch(at)) {
                    if i >= op_i {
                        let branch = Index(i + len);
                        self.set_branch(at, branch);
                    }
                    return Ok(BackTransform::Transformed);
                }
                Err(Error::IncompatiblePositions(pos.clone(), self.clone()))
            }

            Move(from, to) => {
                if from == to {
                    return Ok(BackTransform::Transformed);
                }

                if to.is_root() {
                    println!("to is root");
                    self.replace_head(0, from.clone());
                    return Ok(BackTransform::Transformed);
                }

                if from.is_ancestor(to) {
                    return Err(Error::InvalidOperation(Move(from.clone(), to.clone())));
                }

                let to_parent_is_ancestor = to.is_parent_ancestor(&self);
                let from_parent_is_ancestor = from.is_parent_ancestor(&self);
                let move_under_same_parent = to_parent_is_ancestor
                    && from.is_sibling(to);

                if move_under_same_parent {
                    match (self.branch(to.len() - 1), from.leaf(), to.leaf()) {
                        (Index(i), Index(fi), Index(mut ti)) => {
                            if *fi < ti {
                                ti -= 1;
                            }

                            if *i == ti {
                                if self.len() != to.len()
                                    || position_type == PositionType::Exact
                                    || position_type == PositionType::Change
                                {
                                    let branch = Index(*fi);
                                    self.set_branch(to.len() - 1, branch);
                                    Ok(BackTransform::Transformed)
                                } else if let Some(Move(_, map_to)) = map {
                                    *self = map_to.clone();
                                    Ok(BackTransform::Mapped)
                                } else {
                                    Ok(BackTransform::Transformed)
                                }
                            } else {
                                let mut new_i = *i;

                                if i >= fi {
                                    new_i += 1;
                                }

                                if *i > ti {
                                    new_i -= 1;
                                }

                                let branch = Index(new_i);
                                self.set_branch(to.len() - 1, branch);
                                Ok(BackTransform::Transformed)
                            }
                        }

                        (Field(f), Field(ff), Field(tf)) => {
                            if f == tf {
                                let branch = Field(ff.clone());
                                self.set_branch(from.len() - 1, branch);
                                Ok(BackTransform::Transformed)
                            } else if f == ff {
                                if let Some(Move(map_from, _)) = map {
                                    self.replace_head(to.len(), map_from.clone());
                                    Ok(BackTransform::Mapped)
                                } else {
                                    Ok(BackTransform::None)
                                }
                            } else {
                                Ok(BackTransform::Transformed)
                            }
                        }

                        _ => Err(Error::InvalidOperation(Move(from.clone(), to.clone())))
                    }
                } else if to_parent_is_ancestor {
                    match (self.branch(to.len()- 1), to.leaf()) {
                        (Index(i), Index(ti)) => {
                            if i == ti {
                                if position_type == PositionType::Exact
                                    || position_type == PositionType::Change
                                {
                                    self.replace_head(to.len() - 1, from.clone());
                                    Ok(BackTransform::Transformed)
                                } else if let Some(Move(_, map_to)) = map {
                                    *self = map_to.clone();
                                    Ok(BackTransform::Mapped)
                                } else {
                                    Ok(BackTransform::Transformed)
                                }
                            } else {
                                if i > ti {
                                    let branch = Index(i - 1);
                                    self.set_branch(to.len() - 1, branch);
                                }
                                Ok(BackTransform::Transformed)
                            }
                        }

                        (Field(f), Field(tf)) => {
                            if f == tf {
                                self.replace_head(to.len(), from.clone());
                            }
                            Ok(BackTransform::Transformed)
                        }

                        _ => Err(Error::InvalidOperation(Move(from.clone(), to.clone())))
                    }
                } else if from_parent_is_ancestor {
                    match (self.branch(to.len()- 1), from.leaf()) {
                        (Index(i), Index(fi)) => {
                            if i >= fi {
                                let branch = Index(i + 1);
                                self.set_branch(from.len() - 1, branch);
                            }
                            Ok(BackTransform::Transformed)
                        }

                        (Field(f), Field(ff)) => {
                            if f == ff {
                                if let Some(Move(map_from, _)) = map {
                                    self.replace_head(to.len(), map_from.clone());
                                    Ok(BackTransform::Mapped)
                                } else {
                                    Ok(BackTransform::None)
                                }
                            } else {
                                Ok(BackTransform::Transformed)
                            }
                        }

                        _ => Err(Error::InvalidOperation(Move(from.clone(), to.clone())))
                    }
                } else {
                    Ok(BackTransform::Transformed)
                }
            }

            Set(pos, _) => {
                if self.len() >= pos.len() && self.0[..pos.len()] == pos.0 {
                    if let Some(Set(map_pos, _)) = map {
                        self.replace_head(pos.len(), map_pos.clone());
                        Ok(BackTransform::Mapped)
                    } else {
                        Ok(BackTransform::None)
                    }
                } else {
                    Ok(BackTransform::Transformed)
                }
            }

            _ => Ok(BackTransform::Transformed)
        }
    }

    pub fn transform_forward(
        &mut self,
        op: &Operation,
        position_type: PositionType,
        extend_range: bool
    ) -> Result<bool, Error> {
        use Branch::Index;
        use Branch::Field;
        use Operation::*;

        match op {
            Insert(pos, values) => {
                if self.is_root() {
                    return Ok(true);
                }
                if pos.is_root() {
                    return Err(Error::InvalidOperation(Insert(pos.clone(), values.clone())));
                }
                if !pos.is_parent_ancestor(&self) {
                    return Ok(true);
                }
                let at = pos.len() - 1;
                if let (Index(op_i), Index(i)) = (pos.leaf(), self.branch(at)) {
                    if i > op_i
                        || (
                            i == op_i
                            && (
                                self.len() != pos.len()
                                || position_type != PositionType::RangeEnd
                                || extend_range
                            )
                        )
                    {
                        let branch = Index(i + values.len());
                        self.set_branch(at, branch);
                    }
                    Ok(true)
                } else {
                    Err(Error::IncompatiblePositions(self.clone(), pos.clone()))
                }
            }

            Delete(pos, len) => {
                if self.is_root() {
                    return Ok(true);
                }
                if pos.is_root() {
                    return Err(Error::InvalidOperation(Delete(pos.clone(), *len)));
                }
                if !pos.is_parent_ancestor(&self) {
                    return Ok(true);
                }
                let at = pos.len() - 1;
                if let (Index(op_i), Index(i)) = (pos.leaf(), self.branch(at)) {
                    if *i < op_i + len {
                        if self.len() == pos.len() {
                            use PositionType::*;
                            match position_type {
                                RangeStart | RangeEnd => {
                                    if i > op_i {
                                        self.set_branch(at, Index(*op_i))
                                    }
                                    Ok(true)
                                }
                                Anchor => Ok(i <= op_i),
                                Exact | Change => Ok(i < op_i)
                            }
                        } else {
                            Ok(i < op_i)
                        }
                    } else {
                        let branch = Index(i - len);
                        self.set_branch(at, branch);
                        Ok(true)
                    }
                } else {
                    Err(Error::IncompatiblePositions(self.clone(), pos.clone()))
                }
            }

            InsertChars(pos, chars) => {
                if self.is_root() {
                    return Ok(true);
                }
                if pos.is_root() {
                    return Err(Error::InvalidOperation(InsertChars(pos.clone(), chars.clone())));
                }
                if !pos.is_sibling(&self) {
                    return Ok(true);
                }
                let at = pos.len() - 1;
                if let (Index(op_i), Index(i)) = (pos.leaf(), self.branch(at)) {
                    if i > op_i
                        || (
                            i == op_i
                            && (
                                position_type != PositionType::RangeEnd
                                || extend_range
                            )
                        )
                    {
                        let branch = Index(i + chars.len());
                        self.set_branch(at, branch);
                    }
                    Ok(true)
                } else {
                    Err(Error::IncompatiblePositions(self.clone(), pos.clone()))
                }
            }

            DeleteChars(pos, len) => {
                if self.is_root() {
                    return Ok(true);
                }
                if pos.is_root() {
                    return Err(Error::InvalidOperation(Delete(pos.clone(), *len)));
                }
                if !pos.is_sibling(&self) {
                    return Ok(true);
                }
                let at = pos.len() - 1;
                if let (Index(op_i), Index(i)) = (pos.leaf(), self.branch(at)) {
                    if *i < op_i + len {
                        use PositionType::*;
                        match position_type {
                            RangeStart | RangeEnd => {
                                if i > op_i {
                                    self.set_branch(at, Index(*op_i))
                                }
                                Ok(true)
                            }
                            Anchor =>  Ok(i <= op_i),
                            Exact | Change => Ok(i < op_i)
                        }
                    } else {
                        let branch = Index(i - len);
                        self.set_branch(at, branch);
                        Ok(true)
                    }
                } else {
                    Err(Error::IncompatiblePositions(self.clone(), pos.clone()))
                }
            }

            Move(from, to) => {
                if from == to {
                    return Err(Error::InvalidOperation(Move(from.clone(), to.clone())));
                }

                if from.is_ancestor(to) {
                    return Err(Error::InvalidOperation(Move(from.clone(), to.clone())));
                }

                if from.is_root() {
                    return Err(Error::InvalidOperation(Move(from.clone(), to.clone())));
                }

                if to.is_root() {
                    if from.is_ancestor_or_equal(&self)
                        && (
                            self.len() != from.len()
                            || position_type == PositionType::Exact
                            || position_type == PositionType::Change
                        )
                    {
                        self.replace_head(from.len(), to.clone());
                        return Ok(true);
                    }
                    return Ok(false);
                }

                if from.is_parent_ancestor(&self) {
                    let from_end = from.len() - 1;
                    match (self.branch(from_end), from.leaf()) {
                        (Index(i), Index(from_i)) => {
                            if to.is_sibling(from) {
                                if let Index(to_i) = to.leaf() {
                                    if i == from_i && (
                                        from.len() != self.len()
                                        || (
                                            position_type == PositionType::Exact
                                            || position_type == PositionType::Change
                                        )
                                    ) {
                                        let new_i = if from_i < to_i {
                                            to_i - 1
                                        } else {
                                            *to_i
                                        };
                                        self.set_branch(from.len() - 1, Index(new_i));
                                        Ok(true)
                                    } else {
                                        let mut new_i = *i;
                                        if i > from_i {
                                            new_i -= 1;
                                        }
                                        if i > to_i
                                            || (
                                                i == to_i
                                                && (
                                                    self.len() != to.len()
                                                    || position_type != PositionType::RangeEnd
                                                    || extend_range
                                                )
                                            )
                                        {
                                            new_i += 1;
                                        }
                                        let branch = Index(new_i);
                                        self.set_branch(from_end, branch);
                                        Ok(true)
                                    }
                                } else {
                                    Err(Error::IncompatiblePositions(self.clone(), from.clone()))
                                }
                            } else if i == from_i && (
                                position_type == PositionType::Exact
                                || position_type == PositionType::Change
                            ) {
                                self.replace_head(from.len(), to.clone());
                                Ok(true)
                            } else if i > from_i {
                                let branch = Index(i - 1);
                                self.set_branch(from_end, branch);
                                Ok(true)
                            } else {
                                Ok(true)
                            }
                        }

                        (Field(f), Field(from_f)) => {
                            if f == from_f {
                                self.replace_head(from.len(), to.clone());
                                Ok(true)
                            } else if to.is_parent_ancestor(from) {
                                Ok(false)
                            } else {
                                Ok(true)
                            }
                        }

                        _ => Err(Error::IncompatiblePositions(self.clone(), from.clone()))
                    }
                } else if to.is_parent_ancestor(&self) {
                    let at = to.len() - 1;
                    match (self.branch(at), to.leaf()) {
                        (Index(i), Index(to_i)) => {
                            if i > to_i
                                || (
                                    i == to_i
                                    && (
                                        self.len() != to.len()
                                        || position_type != PositionType::RangeEnd
                                        || extend_range
                                    )
                                )
                            {
                                let branch = Index(i + 1);
                                self.set_branch(at, branch);
                            }
                            Ok(true)
                        }

                        (Field(f), Field(from_f)) => {
                            if self.len() == to.len() && position_type == PositionType::Exact {
                                Ok(true)
                            } else {
                                Ok(f != from_f)
                            }
                        }

                        _ => Err(Error::IncompatiblePositions(self.clone(), to.clone()))
                    }
                } else {
                    Ok(true)
                }
            }

            Set(pos, _) => {
                if pos.is_ancestor_or_equal(&self)
                    && (
                        self.len() != pos.len()
                        || position_type == PositionType::Change
                    )
                {
                    return Ok(false);
                }
                Ok(true)
            }

            _ => Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    use super::*;
    use super::Branch::*;
    use super::Operation::*;

    /// An anchor position should not change when items are inserted after it.
    #[test]
    fn transform_forward_insert_after_anchor_index() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(1)
        ]);
        let op = Operation::Insert(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), vec![Value::Null, Value::Null]);
        assert!(pos.transform_forward(&op, PositionType::Anchor, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(1)
        ]));
    }

    /// An anchor position should be shift to the right when items are inserted
    /// at the same position.
    #[test]
    fn transform_forward_insert_at_anchor_index() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Insert(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), vec![Value::Null, Value::Null]);
        assert!(pos.transform_forward(&op, PositionType::Anchor, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]));
    }

    /// Inserting items at the end of a delete range should extend the range.
    #[test]
    fn transform_forward_insert_at_range_end_extend() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Insert(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), vec![Value::Null, Value::Null]);
        assert!(pos.transform_forward(&op, PositionType::RangeStart, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]));
    }

    /// An anchor located at start of a delete range should remain the same.
    #[test]
    fn transform_forward_delete_at_anchor() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Delete(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(pos.transform_forward(&op, PositionType::Anchor, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// An exact position located at start of a delete range should not exist.
    #[test]
    fn transform_forward_delete_at_exact() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Delete(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(!pos.transform_forward(&op, PositionType::Exact, false).unwrap());
    }

    /// An anchor position that is located within a delete range should not
    /// remain after the operation.
    #[test]
    fn transform_forward_delete_anchor_within_range() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(3)
        ]);
        let op = Operation::Delete(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(!pos.transform_forward(&op, PositionType::Anchor, false).unwrap());
    }

    /// An anchor position that's located after the delete range should be
    /// shifted to the left by the length of the delete range.
    #[test]
    fn transform_forward_delete_just_before_anchor() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(6)
        ]);
        let op = Operation::Delete(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(pos.transform_forward(&op, PositionType::Anchor, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// Deleting a range position should set it to the start of the delete range.
    #[test]
    fn transform_forward_delete_range_start() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]);
        let op = Operation::Delete(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(pos.transform_forward(&op, PositionType::RangeStart, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// Deleting an exact position should delete it.
    #[test]
    fn transform_forward_delete_start_at_exact() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Delete(Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(!pos.transform_forward(&op, PositionType::Exact, false).unwrap());
    }

    /// Move an exact position.
    #[test]
    fn transform_forward_move_exact_index_under_same_parent() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(pos.transform_forward(&op, PositionType::Exact, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(3)
        ]));
    }

    /// Move a sibling of an exact position under the same parent.
    #[test]
    fn transform_forward_move_sibling_of_exact_index_under_same_parent_between() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(3)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(pos.transform_forward(&op, PositionType::Exact, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// Move a sibling of an exact position under the same parent.
    #[test]
    fn transform_forward_move_to_exact_sibling() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(pos.transform_forward(&op, PositionType::Exact, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]));
    }

    /// Move a sibling of an exact position under the same parent.
    #[test]
    fn transform_forward_move_to_before_exact_sibling() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(5)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(pos.transform_forward(&op, PositionType::Exact, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(5)
        ]));
    }

    /// Move a anchor position.
    #[test]
    fn transform_forward_move_anchor() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(pos.transform_forward(&op, PositionType::Anchor, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// Move to the end of a range but don't extend the range.
    #[test]
    fn transform_forward_move_to_range_end_not_extend_range() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(pos.transform_forward(&op, PositionType::RangeStart, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]));
    }

    /// Move to the end of a range and extend the range.
    #[test]
    fn transform_forward_move_to_range_end_extend_range() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(8)
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(pos.transform_forward(&op, PositionType::RangeStart, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(5)
        ]));
    }

    #[test]
    fn transform_forward_move_to_change_field() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("x")),
                Field(String::from("b"))
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ])
        );
        assert!(!pos.transform_forward(&op, PositionType::Change, false).unwrap());
    }

    #[test]
    fn transform_forward_move_to_anchor_field_parent() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(0)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("x")),
                Field(String::from("b"))
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ])
        );
        assert!(!pos.transform_forward(&op, PositionType::Anchor, false).unwrap());
    }

    #[test]
    fn transform_forward_move_to_exact_field() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("x")),
                Field(String::from("b"))
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ])
        );
        assert!(pos.transform_forward(&op, PositionType::Exact, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]));
    }

    #[test]
    fn transform_forward_move_to_exact_field_parent() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(0)
        ]);
        let op = Move(
            Position(vec![
                Field(String::from("x")),
                Field(String::from("b"))
            ]),
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ])
        );
        assert!(!pos.transform_forward(&op, PositionType::Exact, false).unwrap());
    }

    #[test]
    fn transform_forward_set_exact_field() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]);
        let op = Set(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ]),
            Value::Null
        );
        assert!(pos.transform_forward(&op, PositionType::Exact, false).unwrap());
        assert_eq!(pos, Position(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]));
    }

    #[test]
    fn transform_forward_set_change_field() {
        let mut pos = Position(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]);
        let op = Set(
            Position(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ]),
            Value::Null
        );
        assert!(!pos.transform_forward(&op, PositionType::Change, false).unwrap());
    }
}
