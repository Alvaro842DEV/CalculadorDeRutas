use crate::structures::Grafo;
use crate::algorithms::heuristics::adaptive_heuristic;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct State {
    cost: f64,
    position: usize,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn bidirectional_search(graph: &Grafo, start: usize, goal: usize) -> Option<(Vec<(usize, usize)>, f64)> {
    let mut forward_heap = BinaryHeap::new();
    let mut backward_heap = BinaryHeap::new();
    let mut forward_cost = vec![f64::INFINITY; graph.num_vert()];
    let mut backward_cost = vec![f64::INFINITY; graph.num_vert()];
    let mut forward_visited = HashSet::new();
    let mut backward_visited = HashSet::new();
    let mut forward_came_from = vec![None; graph.num_vert()];
    let mut backward_came_from = vec![None; graph.num_vert()];

    forward_heap.push(State { cost: 0.0, position: start });
    backward_heap.push(State { cost: 0.0, position: goal });
    forward_cost[start] = 0.0;
    backward_cost[goal] = 0.0;

    let mut best_cost = f64::INFINITY;
    let mut best_meeting_point = None;

    while !forward_heap.is_empty() && !backward_heap.is_empty() {
        if let Some((new_meeting_point, new_cost)) = expand_heap(graph, &mut forward_heap, &mut forward_cost, &mut forward_visited, &mut forward_came_from, &backward_cost, goal, true) {
            if new_cost < best_cost {
                best_cost = new_cost;
                best_meeting_point = Some(new_meeting_point);
            }
        }

        if let Some((new_meeting_point, new_cost)) = expand_heap(graph, &mut backward_heap, &mut backward_cost, &mut backward_visited, &mut backward_came_from, &forward_cost, start, false) {
            if new_cost < best_cost {
                best_cost = new_cost;
                best_meeting_point = Some(new_meeting_point);
            }
        }

        if forward_heap.peek().map_or(f64::INFINITY, |s| s.cost) +
           backward_heap.peek().map_or(f64::INFINITY, |s| s.cost) >= best_cost {
            break;
        }
    }

    best_meeting_point.map(|meeting_point| {
        reconstruct_path(meeting_point, &forward_came_from, &backward_came_from, start, goal, graph)
    })
}

fn expand_heap(
    graph: &Grafo,
    heap: &mut BinaryHeap<State>,
    cost: &mut [f64],
    visited: &mut HashSet<usize>,
    came_from: &mut [Option<usize>],
    other_cost: &[f64],
    target: usize,
    _is_forward: bool,
) -> Option<(usize, f64)> {
    if let Some(State { cost: current_cost, position }) = heap.pop() {
        if visited.insert(position) {
            if other_cost[position] < f64::INFINITY {
                let total_cost = current_cost + other_cost[position];
                return Some((position, total_cost));
            }

            for &(neighbor, _edge_cost) in graph.neighbours(position) {
                let next_cost = current_cost + 1.0;
                if next_cost < cost[neighbor] {
                    cost[neighbor] = next_cost;
                    came_from[neighbor] = Some(position);
                    let h = adaptive_heuristic(graph, neighbor, target, next_cost);
                    let priority = next_cost + h;
                    heap.push(State { cost: priority, position: neighbor });
                }
            }
        }
    }
    None
}

fn reconstruct_path(
    meeting_point: usize,
    forward_came_from: &[Option<usize>],
    backward_came_from: &[Option<usize>],
    start: usize,
    goal: usize,
    graph: &Grafo,
) -> (Vec<(usize, usize)>, f64) {
    let mut path = Vec::new();
    let mut current = meeting_point;

    while current != start {
        path.push(*graph.node(current).unwrap());
        current = forward_came_from[current].unwrap();
    }
    path.push(*graph.node(start).unwrap());
    path.reverse();

    current = backward_came_from[meeting_point].unwrap_or(goal);
    while current != goal {
        path.push(*graph.node(current).unwrap());
        current = backward_came_from[current].unwrap();
    }
    if meeting_point != goal {
        path.push(*graph.node(goal).unwrap());
    }

    let total_cost = (path.len() - 1) as f64;

    (path, total_cost)
}
