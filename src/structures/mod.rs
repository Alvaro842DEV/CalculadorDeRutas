use std::f64;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Grafo {
    edges: Vec<Vec<(usize, f64)>>,
    nodes: Vec<(usize, usize)>,
    regions: Vec<usize>,
    region_boundaries: HashMap<usize, HashSet<usize>>,
}

impl Grafo {
    pub fn new(n: usize, nodes: Vec<(usize, usize)>) -> Self {
        Grafo {
            edges: vec![Vec::new(); n],
            nodes,
            regions: vec![0; n],
            region_boundaries: HashMap::new(),
        }
    }

    pub fn num_vert(&self) -> usize {
        self.nodes.len()
    }

    pub fn add_edge(&mut self, u: usize, v: usize, cost: f64) {
        self.edges[u].push((v, cost));
        self.edges[v].push((u, cost));
    }

    pub fn neighbours(&self, v: usize) -> &[(usize, f64)] {
        &self.edges[v]
    }

    pub fn node(&self, idx: usize) -> Option<&(usize, usize)> {
        self.nodes.get(idx)
    }

    pub fn set_region(&mut self, node: usize, region: usize) {
        self.regions[node] = region;
    }

    pub fn get_region(&self, node: usize) -> usize {
        self.regions[node]
    }

    pub fn add_region_boundary(&mut self, region: usize, node: usize) {
        self.region_boundaries.entry(region).or_default().insert(node);
    }
}
