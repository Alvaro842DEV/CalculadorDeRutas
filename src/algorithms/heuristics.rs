use crate::structures::Grafo;

pub fn adaptive_heuristic(graph: &Grafo, current: usize, goal: usize, cost_so_far: f64) -> f64 {
    let (x1, y1) = *graph.node(current).unwrap();
    let (x2, y2) = *graph.node(goal).unwrap();
    let dx = (x1 as f64 - x2 as f64).abs();
    let dy = (y1 as f64 - y2 as f64).abs();

    let manhattan_distance = dx + dy;
    let adaptation_factor = 1.0 + (cost_so_far / manhattan_distance).min(0.1);
    let region_penalty = if graph.get_region(current) != graph.get_region(goal) { 1.1 } else { 1.0 };

    manhattan_distance * adaptation_factor * region_penalty
}
