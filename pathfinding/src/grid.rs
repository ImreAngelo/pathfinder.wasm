use wasm_bindgen::prelude::*;

use crate::{bfs, traits::WeightedGraph};

/// A simple grid-based graph for pathfinding
#[wasm_bindgen]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub(crate) walkable: Vec<bool>,
}

impl Grid {
    fn idx(&self, x: u32, y: u32) -> u32 { y * self.width + x }
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height
    }
}

impl WeightedGraph for Grid {
    fn neighbors(&self, node: u32) -> Vec<(u32, f32)> {
        let mut out = Vec::new();
        
        let x = node % self.width;
        let y = node / self.width;

        // 4-directional movement
        // TODO: figure out best way to switch between 4 and 8 directional movement live
        const DIRS: [(i32,i32);4] = [(1,0),(-1,0),(0,1),(0,-1)];
        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if self.in_bounds(nx, ny) {
                let id = self.idx(nx as u32, ny as u32);
                let cost = if self.walkable[node as usize] { 1.0 } else { -1.0 };
                out.push((id, cost));
            }
        }

        out
    }
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Grid {
        let mut walkable = Vec::new();

        for _y in 0..height {
            for _x in 0..width {
                walkable.push(true);
            }
        }

        Grid { width, height, walkable }
    }

    pub fn from_walkable(width: u32, height: u32, walkable: &[u8]) -> Result<Grid, JsValue> {
        let n = (width as usize) * (height as usize);
        if walkable.len() != n {
            return Err(JsValue::from_str("walkable length must be w*h"));
        }

        // Convert UInt8Array to Vec<bool>
        Ok(Grid { width, height, walkable: walkable.into_iter().map(|v| *v != 0).collect() })
    }

    /// Set walkable
    pub fn set_walkable(&mut self, x: u32, y: u32, walkable: bool) {
        let idx = self.idx(x, y);
        if let Some(node) = self.walkable.get_mut(idx as usize) {
            *node = walkable
        }
    }

    pub fn batch_set_walkable(&mut self, walkable: &[u8]) -> Result<(), JsValue> {
        if self.walkable.len() != walkable.len() {
            return Err(JsValue::from_str("walkable length must be w*h"));
        }

        // Convert UInt8Array to Vec<bool>
        self.walkable = walkable.into_iter().map(|v| *v != 0).collect();
        Ok(())
    }

    /// Returns specific node
    pub fn get_node(&self, x: u32, y: u32) -> Option<bool> {
        let idx = self.idx(x, y);
        self.walkable.get(idx as usize).copied()
    }

    /// Returns all nodes
    pub fn nodes(&self) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut id = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                nodes.push(Node {
                    id,
                    x,
                    y,
                    walkable: self.walkable[id]
                });

                id += 1;
            }
        }

        nodes
    }

    /// Returns the shortest path found with Breadth-First Search
    pub fn bfs(&self, start: u32, goal: u32) -> Option<Vec<u32>> {
        bfs::bfs(self, start, goal)
    }

    /// Returns the shortest path found with Jump-Point Search
    pub fn jps(&self, start: u32, goal: u32) -> Option<Box<[u32]>> {
        // jps(self, start, goal)   // grid-only algorithm
        None
    }
}


#[wasm_bindgen]
pub struct Node {
    pub id: usize,
    pub x: u32,
    pub y: u32,
    pub walkable: bool,
}

