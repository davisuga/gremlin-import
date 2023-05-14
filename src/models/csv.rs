use serde::Deserialize;

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Node {
    pub properties: HashMap<String, String>,
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub relationship: String,
}
