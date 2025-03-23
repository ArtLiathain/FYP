use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use maze_library::environment::environment::{Coordinate, Environment};

pub fn dijkstra_solve(env: &mut Environment) {
    let mut main_stack = BinaryHeap::new(); // Priority queue
    let mut path_map: HashMap<Coordinate, (usize, Coordinate)> = HashMap::new();
    let mut visited = HashSet::new(); // Track visited cells

    path_map.insert(env.maze.get_starting_point(), (0, env.maze.get_starting_point()));
    main_stack.push(Reverse((0, env.maze.get_starting_point()))); // Push starting point

    let weighted_graph = env.maze.convert_to_weighted_graph();

    while let Some(Reverse((distance, current))) = main_stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        let neighbors: Vec<(Coordinate, usize)> = weighted_graph
            .get(&current)
            .unwrap()
            .iter()
            .map(|(k, &v)| (env.maze.move_from(k, &current, Some(v)).unwrap(), v))
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

    let mut head = env.maze.end;
    let (final_steps, _) = path_map.get(&env.maze.end).unwrap();
    env.steps = *final_steps;

    let mut path_followed = vec![];
    while head != env.maze.start {
        if let Some((_, temp)) = path_map.get(&head) {
            path_followed.push(head);
            head = *temp;
        } else {
            panic!("Failed to reconstruct the path: Node {:?} is unreachable.", head);
        }
    }
    path_followed.push(env.maze.start);
    path_followed.reverse();
    env.path_followed = path_followed;
}
