pub mod maze_solve {
    use std::collections::HashSet;

    use crate::{environment::environment::Coordinate, maze::maze::{Direction, Maze}};

    pub fn solve_maze_dfs(maze: &mut Maze) -> Vec<Coordinate> {
        let mut stack = vec![(maze.get_starting_point(), 0)]; // Stack for DFS
        let mut visited = HashSet::new(); // Track visited cells
        let mut path = vec![]; // Final path to the goal
        let mut step = 0;
        let end = maze.get_end_point();
        while let Some(current) = stack.pop() {
            if visited.contains(&current.0) {
                continue;
            }

            if step > current.1 {
                // maze.take_step((step-current.1)*2);
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
                // maze.take_step(step-1);
                return path.into_iter().map(|(coords, _)| coords).collect();
            }
            // Explore neighbors
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let neighbor = match maze.move_from(direction, &current.0) {
                    Ok(coordinates) => {coordinates},
                    Err(_) => {continue;}
                };

                if !visited.contains(&neighbor)        // Ensure not visited
                    && !maze.grid[current.0.0][current.0.1]
                        .walls
                        .contains(direction)
                {
                    stack.push((neighbor, step));
                }
            }
        }

        // If no solution exists, return an empty path
        vec![]
    }

    pub fn solve_maze_for_animated_dfs(maze: &mut Maze) -> Vec<(Coordinate, usize)> {
        let mut stack = vec![(maze.get_starting_point(), 0)]; // Stack for DFS
        let mut visited_nodes = vec![]; // Stack for DFS
        let mut visited = HashSet::new(); // Track visited cells
        let mut path: Vec<((usize, usize), usize)> = vec![]; // Final path to the goal
        let mut step = 0;
        let end = maze.get_end_point();

        while let Some(current) = stack.pop() {
            if visited.contains(&current.0) {
                continue;
            }
            // Backtrack path if needed
            if step > current.1 {
                // maze.take_step((step-current.1)*2);
                path = path
                    .into_iter()
                    .filter(|x: &(_, usize)| x.1 < current.1)
                    .collect();
            }
            step = current.1 + 1;
            path.push(current);
            visited_nodes.push(current);
            visited.insert(current.0);

            if current.0 == end {
                // maze.take_step(step-1);
                return visited_nodes;
            }

            // Explore neighbors
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let neighbor = match maze.move_from(direction, &current.0) {
                    Ok(coordinates) => {coordinates},
                    Err(_) => {continue;}
                };

                if !visited.contains(&neighbor)        // Ensure not visited
                    && !maze.grid[current.0.0][current.0.1]
                        .walls
                        .contains(direction)
                {
                    stack.push((neighbor, step));
                }
            }
        }

        // If no solution exists, return an empty path
        vec![]
    }
}