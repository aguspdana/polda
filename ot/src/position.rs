// TODO: This code looks like it's fresh out of Frankenstein's lab.
// Make it more readable!

use crate::Operation;
use crate::Error;

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
pub enum PathType {
    /// A path that can be deleted only if it's within the delete range,
    /// exclusive to the start and end path.
    /// I.e. `Insert`, `InsertChars`, and `Move` target path.
    Anchor,
    /// A path that can't be deleted.
    RangeStart,
    /// A path that can't be deleted.
    RangeEnd,
    /// A path that can be deleted and can be moved.
    /// I.e. `Set` and `Move` initial path.
    Exact,
    /// Like `Exact` but the path no longer exists after it's set.
    Change
}

pub enum BackTransform {
    Transformed,
    Mapped,
    None
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path(pub Vec<Branch>);

impl Path {
    /// # Panic
    /// Panic if `at` is out of range.
    pub fn branch(&self, at: usize) -> &Branch {
        &self.0[at]
    }

    /// Check if `self` is an anchetor of `of`.
    pub fn is_ancestor(&self, of: &Path) -> bool {
        if self.0.len() >= of.0.len() {
            return false;
        }
        self.0 == of.0[..self.len()]
    }

    /// Check if `self` is an anchetor of `of`.
    pub fn is_ancestor_or_equal(&self, of: &Path) -> bool {
        if self.0.len() > of.0.len() {
            return false;
        }
        self.0 == of.0[..self.len()]
    }

    /// Check if the path have the same length, each branch has the same
    /// branch type, and each field branch has the same value.
    pub fn is_compatible(&self, other: &Path) -> bool {
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
    pub fn is_parent_ancestor(&self, of: &Path) -> bool {
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

    pub fn is_sibling(&self, other: &Path) -> bool {
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
    /// Panic if the path is root.
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
            (Insert(path_before, values_before), Insert(path_after, values_after)) => {
                if !path_before.is_parent_ancestor(&self)
                    || values_before.len() != values_after.len()
                {
                    panic!("Invalid mapper");
                }

                if let Index(i) = *self.branch(path_before.len() - 1) {
                    if let Index(i_b) = path_before.leaf() {
                        if let Index(i_a) = path_after.leaf() {
                            self.replace_head(path_before.len(), path_after.clone());
                            if i_b == i_a {
                                return ();
                            }
                            if i < *i_b {
                                panic!("Invalid mapper");
                            }
                            let offset = i - i_b;
                            let new_i = i_a + offset;
                            let branch = Index(new_i);
                            self.set_branch(path_after.len() - 1, branch);
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

            (InsertChars(path_before, chars_before), InsertChars(path_after, chars_after)) => {
                if !path_before.is_sibling(&self)
                    || chars_before.len() != chars_after.len() {
                    panic!("Invalid mapper");
                }

                if let Index(i) = *self.leaf() {
                    if let Index(i_b) = path_before.leaf() {
                        if let Index(i_a) = path_after.leaf() {
                            if i_b == i_a {
                                return ();
                            }
                            if i < *i_b {
                                panic!("Invalid mapper");
                            }
                            let offset = i - i_b;
                            let new_i = i_a + offset;
                            let branch = Index(new_i);
                            self.set_branch(path_after.len() - 1, branch);
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

            (Set(path_before, _), Set(path_after, _)) => {
                if self.len() < path_before.len()
                    || self.0[..path_before.len()] == path_before.0
                {
                    panic!("Invalid mapper");
                }
                self.replace_head(path_before.len(), path_after.clone());
            }

            (_, _) => panic!("Invalid mapper")
        }
    }

    fn replace_head(&mut self, at: usize, head: Path) {
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
        path_type: PathType
    ) -> Result<BackTransform, Error> {
        use Branch::Field;
        use Branch::Index;
        use Operation::*;

        match op {
            Insert(path, values) => {
                if path.is_root() {
                    return Err(Error::InvalidOperation(Insert(path.clone(), values.clone())));
                }
                if !path.is_parent_ancestor(&self) {
                    return Ok(BackTransform::Transformed);
                }
                let at = path.len() - 1;
                if let (Index(op_i), Index(i)) = (path.leaf(), self.branch(at)) {
                    if *i >= op_i + values.len() {
                        let branch = Index(i - values.len());
                        self.set_branch(at, branch);
                        Ok(BackTransform::Transformed)
                    } else if i >= op_i {
                        if let Some(Insert(map_path, _)) = map {
                            if let Index(map_i) = map_path.leaf() {
                                let offset = i - op_i;
                                let new_i = map_i + offset;
                                let branch = Index(new_i);
                                self.replace_head(path.len(), map_path.clone());
                                self.set_branch(map_path.len() - 1, branch);
                                Ok(BackTransform::Mapped)
                            } else {
                                Ok(BackTransform::None)
                            }
                        } else if self.len() == path.len() && (
                            path_type == PathType::Anchor
                            || path_type == PathType::RangeStart
                            || path_type == PathType::RangeEnd
                        ) {
                            *self = path.clone();
                            Ok(BackTransform::Transformed)
                        } else {
                            Ok(BackTransform::None)
                        }
                    } else {
                        Ok(BackTransform::Transformed)
                    }
                } else {
                    Err(Error::IncompatiblePositions(path.clone(), self.clone()))
                }
            }

            Delete(path, len) => {
                if path.is_root() {
                    return Err(Error::InvalidOperation(Delete(path.clone(), *len)));
                }
                if !path.is_parent_ancestor(&self) {
                    return Ok(BackTransform::Transformed);
                }
                let at = path.len() - 1;
                if let (Index(op_i), Index(i)) = (path.leaf(), self.branch(at)) {
                    if i >= op_i {
                        let branch = Index(i + len);
                        self.set_branch(at, branch);
                    }
                    return Ok(BackTransform::Transformed);
                }
                Err(Error::IncompatiblePositions(path.clone(), self.clone()))
            }

            InsertChars(path, chars) => {
                if !path.is_sibling(&self) {
                    return Ok(BackTransform::Transformed);
                }
                if let (Index(op_i), Index(i)) = (path.leaf(), self.leaf()) {
                    if *i >= op_i + chars.len() {
                        let branch = Index(i - chars.len());
                        self.set_branch(self.len() - 1, branch);
                        Ok(BackTransform::Transformed)
                    } else if i >= op_i {
                        if let Some(Insert(map_path, _)) = map {
                            if let Index(map_i) = map_path.leaf() {
                                let offset = i - op_i;
                                let new_i = map_i + offset;
                                let branch = Index(new_i);
                                self.replace_head(path.len(), map_path.clone());
                                self.set_branch(map_path.len() - 1, branch);
                                Ok(BackTransform::Mapped)
                            } else {
                                Ok(BackTransform::None)
                            }
                        } else if self.len() == path.len() && (
                            path_type == PathType::Anchor
                            || path_type == PathType::RangeStart
                            || path_type == PathType::RangeEnd
                        ) {
                            *self = path.clone();
                            Ok(BackTransform::Transformed)
                        } else {
                            Ok(BackTransform::None)
                        }
                    } else {
                        Ok(BackTransform::Transformed)
                    }
                } else {
                    Err(Error::IncompatiblePositions(self.clone(), path.clone()))
                }
            }

            DeleteChars(path, len) => {
                if !path.is_sibling(&self) {
                    return Ok(BackTransform::Transformed);
                }
                let at = path.len() - 1;
                if let (Index(op_i), Index(i)) = (path.leaf(), self.branch(at)) {
                    if i >= op_i {
                        let branch = Index(i + len);
                        self.set_branch(at, branch);
                    }
                    return Ok(BackTransform::Transformed);
                }
                Err(Error::IncompatiblePositions(path.clone(), self.clone()))
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
                                    || path_type == PathType::Exact
                                    || path_type == PathType::Change
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
                                if path_type == PathType::Exact
                                    || path_type == PathType::Change
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

            Set(path, _) => {
                if self.len() >= path.len() && self.0[..path.len()] == path.0 {
                    if let Some(Set(map_path, _)) = map {
                        self.replace_head(path.len(), map_path.clone());
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
        path_type: PathType,
        extend_range: bool
    ) -> Result<bool, Error> {
        use Branch::Index;
        use Branch::Field;
        use Operation::*;

        match op {
            Insert(path, values) => {
                if self.is_root() {
                    return Ok(true);
                }
                if path.is_root() {
                    return Err(Error::InvalidOperation(Insert(path.clone(), values.clone())));
                }
                if !path.is_parent_ancestor(&self) {
                    return Ok(true);
                }
                let at = path.len() - 1;
                if let (Index(op_i), Index(i)) = (path.leaf(), self.branch(at)) {
                    if i > op_i
                        || (
                            i == op_i
                            && (
                                self.len() != path.len()
                                || path_type != PathType::RangeEnd
                                || extend_range
                            )
                        )
                    {
                        let branch = Index(i + values.len());
                        self.set_branch(at, branch);
                    }
                    Ok(true)
                } else {
                    Err(Error::IncompatiblePositions(self.clone(), path.clone()))
                }
            }

            Delete(path, len) => {
                if self.is_root() {
                    return Ok(true);
                }
                if path.is_root() {
                    return Err(Error::InvalidOperation(Delete(path.clone(), *len)));
                }
                if !path.is_parent_ancestor(&self) {
                    return Ok(true);
                }
                let at = path.len() - 1;
                if let (Index(op_i), Index(i)) = (path.leaf(), self.branch(at)) {
                    if *i < op_i + len {
                        if self.len() == path.len() {
                            use PathType::*;
                            match path_type {
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
                    Err(Error::IncompatiblePositions(self.clone(), path.clone()))
                }
            }

            InsertChars(path, chars) => {
                if self.is_root() {
                    return Ok(true);
                }
                if path.is_root() {
                    return Err(Error::InvalidOperation(InsertChars(path.clone(), chars.clone())));
                }
                if !path.is_sibling(&self) {
                    return Ok(true);
                }
                let at = path.len() - 1;
                if let (Index(op_i), Index(i)) = (path.leaf(), self.branch(at)) {
                    if i > op_i
                        || (
                            i == op_i
                            && (
                                path_type != PathType::RangeEnd
                                || extend_range
                            )
                        )
                    {
                        let branch = Index(i + chars.len());
                        self.set_branch(at, branch);
                    }
                    Ok(true)
                } else {
                    Err(Error::IncompatiblePositions(self.clone(), path.clone()))
                }
            }

            DeleteChars(path, len) => {
                if self.is_root() {
                    return Ok(true);
                }
                if path.is_root() {
                    return Err(Error::InvalidOperation(Delete(path.clone(), *len)));
                }
                if !path.is_sibling(&self) {
                    return Ok(true);
                }
                let at = path.len() - 1;
                if let (Index(op_i), Index(i)) = (path.leaf(), self.branch(at)) {
                    if *i < op_i + len {
                        use PathType::*;
                        match path_type {
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
                    Err(Error::IncompatiblePositions(self.clone(), path.clone()))
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
                            || path_type == PathType::Exact
                            || path_type == PathType::Change
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
                                            path_type == PathType::Exact
                                            || path_type == PathType::Change
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
                                                    || path_type != PathType::RangeEnd
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
                                path_type == PathType::Exact
                                || path_type == PathType::Change
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
                                        || path_type != PathType::RangeEnd
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
                            if self.len() == to.len() && path_type == PathType::Exact {
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

            Set(path, _) => {
                if path.is_ancestor_or_equal(&self)
                    && (
                        self.len() != path.len()
                        || path_type == PathType::Change
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

    /// An anchor path should not change when items are inserted after it.
    #[test]
    fn transform_forward_insert_after_anchor_index() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(1)
        ]);
        let op = Operation::Insert(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), vec![Value::Null, Value::Null]);
        assert!(path.transform_forward(&op, PathType::Anchor, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(1)
        ]));
    }

    /// An anchor path should be shift to the right when items are inserted
    /// at the same path.
    #[test]
    fn transform_forward_insert_at_anchor_index() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Insert(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), vec![Value::Null, Value::Null]);
        assert!(path.transform_forward(&op, PathType::Anchor, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]));
    }

    /// Inserting items at the end of a delete range should extend the range.
    #[test]
    fn transform_forward_insert_at_range_end_extend() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Insert(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), vec![Value::Null, Value::Null]);
        assert!(path.transform_forward(&op, PathType::RangeStart, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]));
    }

    /// An anchor located at start of a delete range should remain the same.
    #[test]
    fn transform_forward_delete_at_anchor() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Delete(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(path.transform_forward(&op, PathType::Anchor, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// An exact path located at start of a delete range should not exist.
    #[test]
    fn transform_forward_delete_at_exact() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Delete(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(!path.transform_forward(&op, PathType::Exact, false).unwrap());
    }

    /// An anchor path that is located within a delete range should not
    /// remain after the operation.
    #[test]
    fn transform_forward_delete_anchor_within_range() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(3)
        ]);
        let op = Operation::Delete(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(!path.transform_forward(&op, PathType::Anchor, false).unwrap());
    }

    /// An anchor path that's located after the delete range should be
    /// shifted to the left by the length of the delete range.
    #[test]
    fn transform_forward_delete_just_before_anchor() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(6)
        ]);
        let op = Operation::Delete(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(path.transform_forward(&op, PathType::Anchor, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// Deleting a range path should set it to the start of the delete range.
    #[test]
    fn transform_forward_delete_range_start() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]);
        let op = Operation::Delete(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(path.transform_forward(&op, PathType::RangeStart, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// Deleting an exact path should delete it.
    #[test]
    fn transform_forward_delete_start_at_exact() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Operation::Delete(Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]), 4);
        assert!(!path.transform_forward(&op, PathType::Exact, false).unwrap());
    }

    /// Move an exact path.
    #[test]
    fn transform_forward_move_exact_index_under_same_parent() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(path.transform_forward(&op, PathType::Exact, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(3)
        ]));
    }

    /// Move a sibling of an exact path under the same parent.
    #[test]
    fn transform_forward_move_sibling_of_exact_index_under_same_parent_between() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(3)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(path.transform_forward(&op, PathType::Exact, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// Move a sibling of an exact path under the same parent.
    #[test]
    fn transform_forward_move_to_exact_sibling() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(path.transform_forward(&op, PathType::Exact, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]));
    }

    /// Move a sibling of an exact path under the same parent.
    #[test]
    fn transform_forward_move_to_before_exact_sibling() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(5)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(path.transform_forward(&op, PathType::Exact, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(5)
        ]));
    }

    /// Move a anchor path.
    #[test]
    fn transform_forward_move_anchor() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(path.transform_forward(&op, PathType::Anchor, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(2)
        ]));
    }

    /// Move to the end of a range but don't extend the range.
    #[test]
    fn transform_forward_move_to_range_end_not_extend_range() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(2)
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(path.transform_forward(&op, PathType::RangeStart, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]));
    }

    /// Move to the end of a range and extend the range.
    #[test]
    fn transform_forward_move_to_range_end_extend_range() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(4)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(8)
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b")),
                Index(4)
            ])
        );
        assert!(path.transform_forward(&op, PathType::RangeStart, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(5)
        ]));
    }

    #[test]
    fn transform_forward_move_to_change_field() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("x")),
                Field(String::from("b"))
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ])
        );
        assert!(!path.transform_forward(&op, PathType::Change, false).unwrap());
    }

    #[test]
    fn transform_forward_move_to_anchor_field_parent() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(0)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("x")),
                Field(String::from("b"))
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ])
        );
        assert!(!path.transform_forward(&op, PathType::Anchor, false).unwrap());
    }

    #[test]
    fn transform_forward_move_to_exact_field() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("x")),
                Field(String::from("b"))
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ])
        );
        assert!(path.transform_forward(&op, PathType::Exact, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]));
    }

    #[test]
    fn transform_forward_move_to_exact_field_parent() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b")),
            Index(0)
        ]);
        let op = Move(
            Path(vec![
                Field(String::from("x")),
                Field(String::from("b"))
            ]),
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ])
        );
        assert!(!path.transform_forward(&op, PathType::Exact, false).unwrap());
    }

    #[test]
    fn transform_forward_set_exact_field() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]);
        let op = Set(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ]),
            Value::Null
        );
        assert!(path.transform_forward(&op, PathType::Exact, false).unwrap());
        assert_eq!(path, Path(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]));
    }

    #[test]
    fn transform_forward_set_change_field() {
        let mut path = Path(vec![
            Field(String::from("a")),
            Field(String::from("b"))
        ]);
        let op = Set(
            Path(vec![
                Field(String::from("a")),
                Field(String::from("b"))
            ]),
            Value::Null
        );
        assert!(!path.transform_forward(&op, PathType::Change, false).unwrap());
    }
}
