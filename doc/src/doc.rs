use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use crate::node::Node;

pub struct Doc {
    nodes: HashMap<String, Node>,
    index: Vec<String>
}
