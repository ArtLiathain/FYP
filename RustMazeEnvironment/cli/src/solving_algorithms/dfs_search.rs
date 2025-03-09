pub mod maze_solve {
    use std::collections::HashSet;

    use maze_library::{direction::Direction, environment::environment::Environment};


    pub fn solve_maze_dfs(env: &mut Environment) {
        let mut stack = vec![(env.maze.get_starting_point(), 0)]; // Stack for DFS
        let mut visited = HashSet::new(); // Track visited cells
        let mut path = vec![]; // Final path to the goal
        let mut step = 0;
        let end = env.maze.get_end_point();
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
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let neighbor = match env.maze.move_from(direction, &current.0) {
                    Ok(coordinates) => {coordinates},
                    Err(_) => {continue;}
                };

                if !visited.contains(&neighbor)        // Ensure not visited
                    && !env.maze.grid[current.0.0][current.0.1]
                        .walls
                        .contains(direction)
                {
                    stack.push((neighbor, step));
                }
            }
        }
    
    }
}