use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Graph {
    pub(crate) adj: Vec<Vec<(usize, f64)>>,
}

#[wasm_bindgen]
impl Graph {
    /// Create an empty graph with `n` nodes
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize) -> Graph {
        Graph { adj: vec![Vec::new(); n] }
    }

    /// Add an undirected edge with a weight (defaults to 1.0)
    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.add_weighted_edge(a, b, 1.0);
    }

    /// Add an undirected edge with a custom weight
    pub fn add_weighted_edge(&mut self, a: usize, b: usize, weight: f64) {
        self.adj[a].push((b, weight));
        self.adj[b].push((a, weight));
    }

    /// Generate a simple grid graph (like for maze/pathfinding)
    // #[wasm_bindgen]
    pub fn new_grid(width: usize, height: usize) -> Graph {
        let mut g = Graph::new(width * height);
        let idx = |x: usize, y: usize| -> usize { y * width + x };

        for y in 0..height {
            for x in 0..width {
                let i = idx(x, y);
                if x + 1 < width {
                    g.add_edge(i, idx(x + 1, y));
                }
                if y + 1 < height {
                    g.add_edge(i, idx(x, y + 1));
                }
            }
        }
        g
    }

    /// Get neighbors of a node
    pub fn neighbors(&self, node: usize) -> Vec<usize> {
        self.adj[node].iter().map(|(n, _)| *n).collect()
    }


    /// Number of nodes
    pub fn size(&self) -> usize {
        self.adj.len()
    }
}
