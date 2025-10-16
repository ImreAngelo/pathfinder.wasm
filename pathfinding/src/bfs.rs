use wasm_bindgen::prelude::*;
use std::collections::{VecDeque, HashSet};
use serde_wasm_bindgen::{to_value, from_value};

#[wasm_bindgen]
pub fn bfs(start: usize, graph: JsValue) -> Result<JsValue, JsValue> {
    // Deserialize JS array into Vec<Vec<usize>>
    let graph: Vec<Vec<usize>> = from_value(graph).map_err(|e| e.to_string())?;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &neighbor in &graph[node] {
            if visited.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    // Serialize result back to JsValue
    to_value(&order).map_err(|e| e.to_string().into())
}
