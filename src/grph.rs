use std::ops::{AddAssign, IndexMut};

use crate::geometry::point::Point4;
use petgraph::graphmap::UnGraphMap;

#[derive(Default)]
pub struct Grph {
    pub graph: UnGraphMap<usize, ()>,
    pub verts: Vec<Point4>,
}

impl Grph {
    pub fn new() -> Self {
        Grph::default()
    }

    pub fn get_vertex(&self, idx: usize) -> &Point4 {
        &self.verts[idx]
    }

    pub fn add_vertex(&mut self, pt: &Point4) -> usize {
        self.graph.add_node(self.verts.len());
        self.verts.push(*pt);
        self.graph.node_count() - 1
    }

    pub fn rem_edge(&mut self, idx1: usize, idx2: usize) {
        self.graph.remove_edge(idx1, idx2);
    }

    pub fn add_edge(&mut self, idx1: usize, idx2: usize) {
        self.graph.add_edge(idx1, idx2, ());
    }

    pub fn move_vertex_by(&mut self, idx: usize, pt: &Point4) {
        self.verts.index_mut(idx).add_assign(*pt);
    }

    pub fn move_vertex_to(&mut self, idx: usize, pt: &Point4) {
        self.verts[idx] = *pt;
    }
}
