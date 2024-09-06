# Routes calculator

Modern algorithm that seeks to improve and be more efficient than A\* and Dijkstra in large and complex graphs.

# Proposed Algorithm Approach: “Exploration by Expansive Frontiers”.

This algorithm uses a combination of exploration based on dynamic frontiers and a local optimization, avoiding an exhaustive search of all nodes as in Dijkstra or A\*. Instead of using a single source and priority queue, the idea is:

1. Create multiple dynamic boundaries that expand towards the objective.
2. Locally evaluate connections between sub-frontiers.
3. Combine exploration with adaptive network partitioning.

# Basic Pseudocode

1. Initialize multiple boundaries from the start node.
2. Divide the network into “expansion zones” based on their connectivity.
3. Expand nodes within the boundaries until the target node is reached.
4. Evaluate connection costs between zones, avoiding visiting nodes that are already within other boundaries.
5. Continue until a path is found that connects the source node and the target node.
