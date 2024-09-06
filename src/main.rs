use std::collections::{HashMap, HashSet, VecDeque};
use std::f64;

mod structures;

#[derive(Debug)]
struct Graph {
    adj_list: HashMap<usize, Vec<(usize, f64)>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: f64) {
        self.adj_list.entry(u).or_insert(Vec::new()).push((v, cost));
        self.adj_list.entry(v).or_insert(Vec::new()).push((u, cost));
    }

    fn neighbors(&self, u: usize) -> &Vec<(usize, f64)> {
        self.adj_list.get(&u).unwrap_or(&Vec::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FrontierNode {
    id: usize,
    cost: f64,  
}

fn advanced_exploratory_search(graph: &Graph, start: usize, goal: usize) -> Option<f64> {
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::new();

    frontier.push_back(FrontierNode { id: start, cost: 0.0 });
    visited.insert(start);

    let mut subzones = HashMap::new();
    let mut min_cost = f64::INFINITY;

    let mut frontier_map = HashMap::new();

    while let Some(current_node) = frontier.pop_front() {
        if current_node.id == goal {
            return Some(current_node.cost);
        }

        for &(neighbor, cost) in graph.neighbors(current_node.id).iter() {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                
                let new_cost = current_node.cost + cost;
                
                let neighbor_node = FrontierNode { id: neighbor, cost: new_cost };
                frontier.push_back(neighbor_node.clone());
                
                subzones.insert(neighbor, new_cost);

                if let Some(existing_cost) = frontier_map.get(&neighbor) {
                    min_cost = min_cost.min(existing_cost + new_cost);
                }
                frontier_map.insert(neighbor, new_cost);
            }
        }

        if min_cost < f64::INFINITY {
            return Some(min_cost);
        }
    }

    None
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.5);
    graph.add_edge(2, 3, 1.0);
    graph.add_edge(0, 3, 3.0);

    let start = 0;
    let goal = 3;

    if let Some(cost) = advanced_exploratory_search(&graph, start, goal) {
        println!("El costo del camino más corto es: {}", cost);
    } else {
        println!("No se encontró un camino.");
    }
}
