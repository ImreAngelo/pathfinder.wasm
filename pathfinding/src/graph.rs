use wasm_bindgen::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::bfs;

#[derive(PartialEq)]
pub struct Node {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub cost: f64,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

#[wasm_bindgen]
pub struct Graph {
    pub(crate) nodes: Vec<Rc<RefCell<Node>>>,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Graph {
        Graph { nodes: Vec::new() }
    }

    pub fn nodes(&self) -> Vec<JSNode> {
        self.nodes.iter().map(|n| node_to_js(n.clone())).collect()
    }

    /// Breadth-First Search for un-weighted graphs
    pub fn bfs(&self, start_id: u32, goal_id: u32) -> Option<Vec<JSNode>> {
        bfs::node(self.nodes.get(start_id as usize)?.clone(), goal_id)
            .map(|n: Vec<NodeRef>| n.into_iter().map(node_to_js).collect())
    }
}


#[wasm_bindgen]
pub struct JSNode {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub cost: f64,
}

#[inline]
fn node_to_js(n: Rc<RefCell<Node>>) -> JSNode {
    let n = n.borrow();
    JSNode { id: n.id, x: n.x, y: n.y, cost: n.cost }
}

pub type NodeRef = Rc<RefCell<Node>>;