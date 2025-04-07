use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use maze_library::{
    environment::environment::{Coordinate, Environment},
    maze::maze::{directional_movement, to_usize_tuple},
};

pub fn dijkstra_solve(env: &Environment, start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
    let mut main_stack = BinaryHeap::new(); // Priority queue
    let mut path_map: HashMap<Coordinate, (usize, Coordinate)> = HashMap::new();
    let mut visited = HashSet::new(); // Track visited cells
    let weighted_graph = &env.weighted_graph;
    let env_visited = &env.visited;
    path_map.insert(start, (0, start));
    main_stack.push(Reverse((0, start))); // Push starting point

    while let Some(Reverse((distance, current))) = main_stack.pop() {
        if visited.contains(&current) || !env_visited.contains_key(&current) {
            continue;
        }

        visited.insert(current);

        let neighbors: Vec<(Coordinate, usize)> = weighted_graph
            .get(&current)
            .unwrap()
            .iter()
            .map(|(k, &v)| (to_usize_tuple(directional_movement(k, &current, v)), v))
            .filter(|(neighbor, _)| !visited.contains(neighbor))
            .collect();

        for (neighbor, weight) in neighbors {
            let tentative_distance = distance + weight;

            if !path_map.contains_key(&neighbor) || tentative_distance < path_map[&neighbor].0 {
                path_map.insert(neighbor, (tentative_distance, current));
                main_stack.push(Reverse((tentative_distance, neighbor)));
            }
        }
    }

    let mut head = end;
    let mut path_followed = vec![];
    while head != start {
        if let Some((_, temp)) = path_map.get(&head) {
            path_followed.push(head);
            head = *temp;
        } else {
            panic!(
                "Failed to reconstruct the path: Node {:?} is unreachable.",
                head
            );
        }
    }
    path_followed.push(start);
    path_followed.reverse();
    path_followed
}
