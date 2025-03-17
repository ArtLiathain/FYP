use std::collections::{HashMap, HashSet};

use maze_library::environment::environment::Environment;

pub fn solve_maze_dfs(env: &mut Environment) {
    let mut stack: Vec<((usize, usize), usize)> = vec![(env.maze.get_starting_point(), 0)]; // Stack for DFS
    let mut visited = HashSet::new(); // Track visited cells
    let mut path = vec![]; // Final path to the goal
    let mut step = 0;
    let end = env.maze.get_end_point();
    let weighted_graph = env.maze.convert_to_weighted_graph();
    while let Some(current) = stack.pop() {
        if visited.contains(&current.0) {
            continue;
        }

        if step > current.1 {
            // env.maze.take_step((step-current.1)*2);
            path = path
                .into_iter()
                .filter(|x: &(_, usize)| x.1 < current.1)
                .collect();
        }
        step = current.1 + 1;
        path.push(current);
        visited.insert(current.0);

        // If we've reached the end, return the path
        if current.0 == end {
            // env.maze.take_step(step-1);
            env.path_followed = path.into_iter().map(|(coords, _)| coords).collect();
            env.steps = env.path_followed.len();
            return;
        }
        // Explore neighbors
        for (direction, steps) in weighted_graph.get(&current.0).unwrap_or(&HashMap::new()) {
            let neighbor = match env.maze.move_from(direction, &current.0, Some(*steps)) {
                Ok(coordinates) => coordinates,
                Err(_) => {
                    continue;
                }
            };

            if !visited.contains(&neighbor) {
                stack.push((neighbor, step));
            }
        }
    }
    println!("NO PATH FOUND");
}
