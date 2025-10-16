use wasm_bindgen::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::graph::Graph;

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    position: usize,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparison for min-heap behavior
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[wasm_bindgen]
pub fn dijkstra(graph: &Graph, start: usize, goal: usize) -> Vec<usize> {
    let mut dist: HashMap<usize, f64> = HashMap::new();
    let mut parent: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0.0);
    heap.push(State { cost: 0.0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
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

        if cost > *dist.get(&position).unwrap_or(&f64::INFINITY) {
            continue;
        }

        for &(neighbor, weight) in &graph.adj[position] {
            let next = State {
                cost: cost + weight,
                position: neighbor,
            };

            if next.cost < *dist.get(&neighbor).unwrap_or(&f64::INFINITY) {
                heap.push(next);
                dist.insert(neighbor, next.cost);
                parent.insert(neighbor, position);
            }
        }
    }

    vec![]
}
