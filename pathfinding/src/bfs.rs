use crate::grid::{ConnectedGraph};

/// Performs Breadth-First Search on a connected graph with un-weighted but walkable nodes
pub fn bfs(_graph: &impl ConnectedGraph, start: u32, goal: u32) -> Option<Box<[u32]>> {
    println!("Running BFS from {} -> {}", start, goal);

    

    Some(Box::new([start, goal]))
}