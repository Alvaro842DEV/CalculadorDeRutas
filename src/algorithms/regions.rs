use crate::structures::Grafo;
use rand::prelude::*;
use std::collections::HashSet;

pub fn partition_graph(graph: &mut Grafo, num_partitions: usize) {
    let mut rng = rand::thread_rng();
    let total_vertices = graph.num_vert();
    let mut unassigned: HashSet<usize> = (0..total_vertices).collect();
    let mut partition_sizes = vec![0; num_partitions];

    let mut seeds: Vec<usize> = unassigned.iter().cloned().collect();
    seeds.shuffle(&mut rng);
    seeds.truncate(num_partitions);

    for (i, &seed) in seeds.iter().enumerate() {
        graph.set_region(seed, i);
        unassigned.remove(&seed);
        partition_sizes[i] += 1;
    }

    while !unassigned.is_empty() {
        let mut next_assignments = Vec::new();

        for &node in &unassigned {
            let neighbors: Vec<usize> = graph.neighbours(node).iter().map(|&(n, _)| n).collect();
            let mut best_partition = 0;
            let mut max_neighbors = 0;

            for p in 0..num_partitions {
                let neighbors_in_partition = neighbors.iter().filter(|&&n| graph.get_region(n) == p).count();
                if neighbors_in_partition > max_neighbors || (neighbors_in_partition == max_neighbors && partition_sizes[p] < partition_sizes[best_partition]) {
                    max_neighbors = neighbors_in_partition;
                    best_partition = p;
                }
            }

            next_assignments.push((node, best_partition));
        }

        for (node, partition) in next_assignments {
            graph.set_region(node, partition);
            partition_sizes[partition] += 1;
            unassigned.remove(&node);

            let is_boundary = graph.neighbours(node).iter()
                .any(|&(neighbor, _)| graph.get_region(neighbor) != partition);
            if is_boundary {
                graph.add_region_boundary(partition, node);
            }
        }
    }
}
