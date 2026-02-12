use wasm_bindgen::prelude::*;

use crate::bfs;

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

/// Implements neighbors as a list of indices
/// TODO: make weighted graph separate, with (un)walkable = -1 etc.
pub trait ConnectedGraph {
    fn neighbors(&self, node: u32, out: &mut Vec<u32>);
    fn is_walkable(&self, node: u32) -> bool;
}

impl ConnectedGraph for Grid {
    fn neighbors(&self, node: u32, out: &mut Vec<u32>) {
        let x = node % self.width;
        let y = node / self.width;

        // 4-neighborhood example
        const DIRS: [(i32,i32);4] = [(1,0),(-1,0),(0,1),(0,-1)];
        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if self.in_bounds(nx, ny) {
                out.push(self.idx(nx as u32, ny as u32));
            }
        }
    }
    fn is_walkable(&self, node: u32) -> bool {
        self.walkable[node as usize]
    }
}

// pub trait WeightedGraph {
//     fn neighbors(&self, node: usize) -> Vec<usize>;
//     fn cost(&self, node: usize) -> f32;
// }

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

    /// Returns specific node
    pub fn get_node(&self, x: u32, y: u32) -> Option<bool> {
        let idx = y * self.width + x;
        self.walkable.get(idx as usize).copied()
    }

    /// Returns all nodes...
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


    pub fn bfs(&self, start: u32, goal: u32) -> Option<Box<[u32]>> {
        bfs::bfs(self, start, goal)
    }

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

