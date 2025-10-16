use wasm_bindgen::prelude::*;
use std::collections::{VecDeque, HashMap, HashSet};
use crate::graph::Graph;


/// Find the shortest path between `start` and `goal` using BFS
#[wasm_bindgen]
pub fn bfs(graph: &Graph, start: usize, goal: usize) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        if node == goal {
            // reconstruct path
            let mut path = vec![goal];
            let mut current = goal;
            while let Some(&p) = parent.get(&current) {
                path.push(p);
                current = p;
            }
            path.reverse();
            return path;
        }

        // iterate over neighbor-weight pairs, ignore weight
        for &(neighbor, _) in &graph.adj[node] {
            if visited.insert(neighbor) {
                parent.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }
    }

    vec![]
}