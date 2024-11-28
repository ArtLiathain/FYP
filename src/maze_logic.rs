pub mod maze_logic {
    use std::collections::HashSet;

    use rand::Rng;

    use crate::maze::maze::{Direction, Maze};

    pub fn random_wilson_maze(width: usize, height: usize) -> Maze {
        let mut maze: Maze = Maze::new(width, height);
        let mut unvisited_nodes: Vec<(usize, usize)> = (0..width)
            .flat_map(|x| (0..height).map(move |y| (x, y)))
            .collect();

        let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
        let end_coordinate = (width / 2, height / 2);
        maze.set_end((width / 2, height / 2));
        maze.set_starting_point(unvisited_nodes[height - 1], Some(&Direction::South));
        let mut current = unvisited_nodes.remove(height - 1);
        while !unvisited_nodes.is_empty() {
            let mut new_path: Vec<((usize, usize), Direction)> = Vec::new();
            let mut new_coordinates: (usize, usize);
            loop {
                let direction = Direction::random();
                new_coordinates = Direction::move_from(&direction, &current);
                if !Maze::in_bounds(&maze, new_coordinates) {
                    continue;
                }
                new_path.push((current, direction));

                let match_index = new_path
                    .iter()
                    .position(|(coordinates, _)| *coordinates == new_coordinates);

                if let Some(index) = match_index {
                    new_path.truncate(index + 1);
                    current = new_path.remove(new_path.len() - 1).0;
                    continue;
                }

                if end_coordinate == new_coordinates || visited_nodes.contains(&new_coordinates) {
                    new_path.push((new_coordinates, direction));

                    break;
                }
                current = new_coordinates;
            }
            Maze::break_walls_for_path(&mut maze, new_path.clone());
            for (coords, _) in new_path {
                visited_nodes.insert(coords);
                unvisited_nodes.retain(|&coord| coord != coords);
            }
            if !unvisited_nodes.is_empty() {
                current =
                    unvisited_nodes.remove(rand::thread_rng().gen_range(0..unvisited_nodes.len()));
            }
        }
        let mut path = Vec::new();
        loop {
            let direction = Direction::random();
            let new_coordinates = Direction::move_from(&direction, &current);
            if !Maze::in_bounds(&maze, new_coordinates) || new_coordinates == current {
                continue;
            }
            path.push((current, direction));
            path.push((new_coordinates, direction));

            break;
        }
        Maze::break_walls_for_path(&mut maze, path);

        maze
    }

    pub fn solve_maze_dfs(maze: &Maze) -> Vec<(usize, usize)> {
        let mut stack = vec![(maze.get_starting_point(), 0)]; // Stack for DFS
        let mut visited = HashSet::new(); // Track visited cells
        let mut path = vec![]; // Final path to the goal
        let mut step = 1;
        let end = maze.get_end_point();
        while let Some(current) = stack.pop() {
            if visited.contains(&current.0) {
                continue;
            }

            if step > current.1 {
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
                return path.into_iter().map(|(coords, _)| coords).collect();
            }
            // Explore neighbors
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let neighbor = direction.move_from(&current.0);

                if maze.in_bounds(neighbor)                // Check bounds
                    && !visited.contains(&neighbor)        // Ensure not visited
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

    pub fn solve_maze_for_animated_dfs(maze: &Maze) -> Vec<((usize, usize), usize)> {
        let mut stack = vec![(maze.get_starting_point(), 0)]; // Stack for DFS
        let mut visited_nodes = vec![]; // Stack for DFS
        let mut visited = HashSet::new(); // Track visited cells
        let mut path = vec![]; // Final path to the goal
        let mut step = 1;
        let end = maze.get_end_point();

        while let Some(current) = stack.pop() {
            if visited.contains(&current.0) {
                continue;
            }
            // Backtrack path if needed
            if step > current.1 {
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
                return visited_nodes;
            }

            // Explore neighbors
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let neighbor = direction.move_from(&current.0);

                if maze.in_bounds(neighbor)                // Check bounds
                    && !visited.contains(&neighbor)        // Ensure not visited
                    && !maze.grid[current.0.0][current.0.1]
                        .walls
                        .contains(direction)
                // Check if there's no wall in the current direction
                {
                    stack.push((neighbor, step));
                }
            }
        }

        // If no solution exists, return an empty path
        vec![]
    }
}
