use std::collections::{HashMap, HashSet, VecDeque};

use crate::{traits::WeightedGraph};

/// Performs Breadth-First Search on a connected graph with un-weighted but walkable nodes
pub fn bfs(graph: &impl WeightedGraph, start: u32, goal: u32) -> Option<Vec<u32>> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    q.push_back(start);
    visited.insert(start);

    while let Some(node) = q.pop_front() {
        if node == goal { // reconstruct path
            let mut path = vec![goal];
            let mut current = goal;

            while let Some(&p) = parent.get(&current) {
                path.push(p);
                current = p;
            }
            
            path.reverse();
            return Some(path);
        }

        // iterate over neighbor-weight pairs, use weight = -1 as unwalkable
        for &(neighbor, w) in &graph.neighbors(node) {
            if w != -1.0 && visited.insert(neighbor) {
                parent.insert(neighbor, node);
                q.push_back(neighbor);
            }
        }
    }

    // path not found
    None 
}