// use std::{cell::RefCell, rc::Rc};

// use wasm_bindgen::prelude::*;

// pub struct Node {
//     pub x: f64,
//     pub y: f64,
//     pub walkable: bool, // TODO: weighted nodes
//     pub neighbors: Vec<Rc<RefCell<Node>>>,
// }

// #[wasm_bindgen]
// pub struct Graph {
//     pub(crate) nodes: Vec<Node>,
// }