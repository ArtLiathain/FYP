use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};
use crate::
    environment::environment::{Coordinate, Environment}
;

pub fn dijkstra_solve(env: &Environment, start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
    let path_map = dijkstra_graph(env, start);

    let mut previous = end;
    let mut path_followed = vec![];
    while previous != start {
        if let Some((_, current)) = path_map.get(&previous) {
            path_followed.push(previous);
            previous = *current;
        } else {
            panic!(
                "Failed to reconstruct the path: Node {:?} is unreachable.",
                previous
            );
        }
    }
    path_followed.push(start);
    path_followed.reverse();
    path_followed
}

pub fn dijkstra_graph(
    env: &Environment,
    start: Coordinate,
) -> HashMap<Coordinate, (usize, Coordinate)> {
    let mut main_stack = BinaryHeap::new(); // Priority queue
    let mut path_map: HashMap<Coordinate, (usize, Coordinate)> = HashMap::new();
    let mut visited = HashSet::new(); // Track visited cells
    let weighted_graph = &env.weighted_graph;
    path_map.insert(start, (0, start));
    main_stack.push(Reverse((0, start))); // Push starting point

    while let Some(Reverse((distance, current))) = main_stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        let neighbors: Vec<(Coordinate, usize)> = weighted_graph
            .get(&current)
            .unwrap_or(&HashMap::new())
            .iter()
            .filter_map(|(direction, &steps)| {
                match env.maze.move_from(direction, &current, steps) {
                    Ok(new_coordinates) => {
                        if visited.contains(&new_coordinates) {
                            return None;
                        }
                        Some((new_coordinates, steps))
                    }
                    Err(_) => None,
                }
            })
            .collect();

        for (neighbor, weight) in neighbors {
            let tentative_distance = distance + weight;

            if !path_map.contains_key(&neighbor) || tentative_distance < path_map[&neighbor].0 {
                path_map.insert(neighbor, (tentative_distance, current));
                main_stack.push(Reverse((tentative_distance, neighbor)));
            }
        }
    }
    path_map
}
