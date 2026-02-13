use std::collections::{HashMap, HashSet, VecDeque};
use crate::{graph::{Node, NodeRef}, traits::WeightedGraph};

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



#[inline]
fn walkable(n: &NodeRef) -> bool {
	n.borrow().cost >= 0.0
}

/// Performs Breadth-First Search on a connected graph
pub fn node(start: NodeRef, goal_id: u32) -> Option<Vec<NodeRef>> {
    let mut q: VecDeque<NodeRef> = VecDeque::new();
    let mut visited: HashSet<u32> = HashSet::new();
    let mut parents: HashMap<u32, NodeRef> = HashMap::new();

    visited.insert(start.borrow().id);
    q.push_back(start);

    while let Some(node) = q.pop_front() {
        if node.borrow().id == goal_id { // reconstruct path
            let mut path = vec![node.clone()];
            let mut current = goal_id;

            while let Some(p) = parents.get(&current) {
                path.push(p.clone());
                current = p.borrow().id;
            }
            
            path.reverse();
            return Some(path);
        }

        // iterate over neighbor-weight pairs, use weight < 0 as unwalkable
        for neighbor in node.borrow().neighbors.iter() {
            let neighbor_id = neighbor.borrow().id;

            if walkable(&neighbor) && visited.insert(neighbor_id) {
                parents.insert(neighbor_id, neighbor.clone());
                q.push_back(neighbor.clone());
            }
        }
    }

    // path not found
    None 
}