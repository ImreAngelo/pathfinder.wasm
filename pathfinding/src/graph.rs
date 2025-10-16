use wasm_bindgen::prelude::*;
use std::collections::{VecDeque, HashSet};

#[wasm_bindgen]
pub struct Graph {
    adj: Vec<Vec<usize>>,
}

#[wasm_bindgen]
impl Graph {
    /// Create an empty graph with `n` nodes
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize) -> Graph {
        Graph { adj: vec![Vec::new(); n] }
    }

    /// Add an undirected edge
    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.adj[a].push(b);
        self.adj[b].push(a);
    }

    /// Generate a simple grid graph (like for maze/pathfinding)
    pub fn new_grid(width: usize, height: usize) -> Graph {
        let mut g = Graph::new(width * height);
        let index = |x: usize, y: usize| -> usize { y * width + x };

        for y in 0..height {
            for x in 0..width {
                let i = index(x, y);
                if x + 1 < width {
                    g.add_edge(i, index(x + 1, y));
                }
                if y + 1 < height {
                    g.add_edge(i, index(x, y + 1));
                }
            }
        }
        g
    }

    /// Perform BFS from a starting node
    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut order = Vec::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &neighbor in &self.adj[node] {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
        order
    }

    /// Get neighbors of a node
    pub fn neighbors(&self, node: usize) -> Vec<usize> {
        self.adj[node].clone()
    }

    /// Number of nodes
    pub fn size(&self) -> usize {
        self.adj.len()
    }
}
