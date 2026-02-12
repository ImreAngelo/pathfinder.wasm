/// Implements neighbors as a list of indices with associated weights
pub trait WeightedGraph {
    fn neighbors(&self, node: u32) -> Vec<(u32, f32)>;
}