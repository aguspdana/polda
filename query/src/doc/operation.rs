use crate::node::Node;
use crate::node::NodeOperation;

#[derive(Debug, Clone)]
pub enum DocOperation {
    InsertNode {
        id: String,
        node: Node
    },
    DeleteNode {
        id: String,
    },
    AlterNode {
        id: String,
        operation: NodeOperation
    },
    InsertIndex {
        index: usize,
        id: String
    },
    DeleteIndex {
        index: usize,
        id: String
    }
}

impl DocOperation {
    /// Map an operation that can't be transformed backward to the transformed
    /// preceding operation.
    ///
    /// # Panic
    /// Panic when the operation can't be mapped.
    pub fn map(self, mapper: &DocOperation) -> DocOperation {
        match (mapper, self) {
            (
                DocOperation::InsertNode { id: new_id, node: _ },
                DocOperation::InsertNode { id, node }
            ) => {
                if &id != new_id {
                    panic!("Can't map InsertNode to InsertNode with a different id");
                }
                DocOperation::InsertNode { id, node }
            }

            (
                DocOperation::InsertNode { id: new_id, node: _ },
                DocOperation::DeleteNode { id }
            ) => {
                if &id != new_id {
                    panic!("Can't map DeleteNode to InsertNode with a different id");
                }
                DocOperation::DeleteNode { id }
            }


            (
                DocOperation::InsertNode { id: new_id, node: _ },
                DocOperation::AlterNode { id, operation }
            ) => {
                if &id != new_id {
                    panic!("Can't map AlterNode to InsertNode with a different id");
                }
                DocOperation::AlterNode { id, operation }
            }

            (
                DocOperation::AlterNode { id: new_id, operation: new_operation },
                DocOperation::AlterNode { id, operation }
            ) => {
                if &id != new_id {
                    panic!("Can't map AlterNode to AlterNode with a different id");
                }
                DocOperation::AlterNode { id, operation: operation.map(new_operation) }
            }

            (
                DocOperation::InsertIndex { index: new_index, id: _ },
                DocOperation::InsertIndex { index: _, id }
            ) => {
                DocOperation::InsertIndex { index: *new_index, id }
            }

            (
                DocOperation::InsertIndex { index: new_index, id: _ },
                DocOperation::DeleteIndex { index: _, id }
            ) => {
                DocOperation::DeleteIndex { index: *new_index, id }
            }

            (a, b) => panic!("Can't map {:?} to {:?}", b, a)
        }
    }

    /// Transform an operation that follows another operation `op2(op1(doc))`
    /// so it can be applied to the same document `new_op2(doc)` without
    /// losing any information.  Return `None` if the backward transformation
    /// can't be transformed forward to get to the original operation.
    pub fn transform_backward(self, preceded_by: &DocOperation) -> Option<DocOperation> {
        match (preceded_by, self) {
            (
                DocOperation::InsertNode { id: pre_id, node: _ },
                DocOperation::InsertNode { id, node }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DocOperation::InsertNode { id, node })
                }
            }

            (
                DocOperation::InsertNode { id: pre_id, node: _ },
                DocOperation::DeleteNode { id }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DocOperation::DeleteNode { id })
                }
            }

            (
                DocOperation::InsertNode { id: pre_id, node: _ },
                DocOperation::AlterNode { id, operation }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DocOperation::AlterNode { id, operation })
                }
            }

            (
                DocOperation::AlterNode { id: pre_id, operation: pre_operation },
                DocOperation::AlterNode { id, operation }
            ) => {
                if &id == pre_id {
                    operation
                        .transform_backward(pre_operation)
                        .map(|operation| DocOperation::AlterNode { id, operation })
                } else {
                    Some(DocOperation::AlterNode { id, operation })
                }
            }

            (
                DocOperation::InsertIndex { index: pre_index, id: _ },
                DocOperation::InsertIndex { mut index, id }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(DocOperation::InsertIndex { index, id })
                }
            }

            (
                DocOperation::InsertIndex { index: pre_index, id: _ },
                DocOperation::DeleteIndex { mut index, id }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(DocOperation::DeleteIndex { index, id })
                }
            }

            (
                DocOperation::DeleteIndex { index: pre_index, id: _ },
                DocOperation::InsertIndex { mut index, id }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(DocOperation::InsertIndex { index, id })
            }

            (
                DocOperation::DeleteIndex { index: pre_index, id: _ },
                DocOperation::DeleteIndex { mut index, id }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(DocOperation::DeleteIndex { index, id })
            }

            (_, op) => Some(op)
        }
    }

    pub fn transform_forward(self, preceded_by: &DocOperation) -> Option<DocOperation> {
        match (preceded_by, self) {
            (
                DocOperation::DeleteNode { id: pre_id },
                DocOperation::DeleteNode { id }
            ) => {
                if &id == pre_id {
                    None
                } else {
                    Some(DocOperation::DeleteNode { id })
                }
            }

            (
                DocOperation::DeleteNode { id: pre_id },
                DocOperation::AlterNode { id, operation }
            ) => {
                if id == *pre_id {
                    None
                } else {
                    Some(DocOperation::AlterNode { id, operation })
                }
            }

            (
                DocOperation::AlterNode { id: pre_id, operation: pre_op },
                DocOperation::AlterNode { id, operation }
            ) => {
                if id == *pre_id {
                    operation.transform_forward(pre_op)
                        .map(|op| DocOperation::AlterNode { id, operation: op })
                } else {
                    Some(DocOperation::AlterNode { id, operation })
                }
            }

            (
                DocOperation::InsertIndex { index: pre_index, id: _ },
                DocOperation::InsertIndex { mut index, id }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(DocOperation::InsertIndex { index, id })
            }

            (
                DocOperation::InsertIndex { index: pre_index, id: _ },
                DocOperation::DeleteIndex { mut index, id }
            ) => {
                if index >= *pre_index {
                    index += 1;
                }
                Some(DocOperation::DeleteIndex { index, id })
            }

            (
                DocOperation::DeleteIndex { index: pre_index, id: _ },
                DocOperation::InsertIndex { mut index, id }
            ) => {
                if index > *pre_index {
                    index -= 1;
                }
                Some(DocOperation::DeleteIndex { index, id })
            }

            (
                DocOperation::DeleteIndex { index: pre_index, id: _ },
                DocOperation::DeleteIndex { mut index, id }
            ) => {
                if index == *pre_index {
                    None
                } else {
                    if index > *pre_index {
                        index -= 1;
                    }
                    Some(DocOperation::DeleteIndex { index, id })
                }
            }

            (_, op) => Some(op)
        }
    }

    pub fn transform_batch(batch: Vec<DocOperation>, preceded_by: &Vec<DocOperation>) -> Vec<DocOperation> {
        let mut transformed_batch: Vec<Option<DocOperation>> = Vec::with_capacity(batch.len());

        batch.iter()
            .enumerate()
            .for_each(|(i, operation)| {
                let prevs = &batch[..i];
                let mut transformed = Some(operation.clone());
                let mut forward_from_index_in_batch = 0;

                // Transform backward against previous operations in the batch.
                for (j, prev_op) in prevs.iter().enumerate().rev() {
                    let op = transformed.take().unwrap();
                    if let Some(op) = op.transform_backward(prev_op) {
                        transformed = Some(op);
                    } else {
                        if let Some(prev_transformed) = &transformed_batch[j].as_ref() {
                            transformed = Some(operation.clone().map(prev_transformed));
                            forward_from_index_in_batch = j + 1;
                        } else {
                            transformed = None;
                        }
                        break;
                    }
                }

                // Transform forward against preceding operations.
                if forward_from_index_in_batch == 0 {
                    for preceding in preceded_by.iter() {
                        if let Some(op) = transformed.take() {
                            transformed = op.transform_forward(preceding);
                        } else {
                            transformed_batch.push(None);
                            return;
                        }
                    }
                }

                // Transform forward against transformed operations in the batch.
                for prev in transformed_batch[forward_from_index_in_batch..].iter() {
                    if let Some(op) = transformed.take() {
                        if let Some(prev) = prev {
                            transformed = op.transform_forward(prev);
                        } else {
                            continue;
                        }
                    } else {
                        transformed_batch.push(None);
                        return;
                    }
                }

                transformed_batch.push(transformed);
            });

        // Remove dropped operations.
        transformed_batch.into_iter()
            .filter(|op| op.is_some())
            .map(|op| op.unwrap())
            .collect()
    }
}
