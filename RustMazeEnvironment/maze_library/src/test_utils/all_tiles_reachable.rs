use std::collections::HashSet;

use crate::{direction::Direction, maze::maze::Maze};

pub fn all_tiles_reachable(maze: &Maze) -> bool {
    let mut stack = vec![maze.get_end_point()]; // Stack for DFS
    let mut visited = HashSet::new(); // Track visited cells
    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        for direction in &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let neighbor = match maze.move_from(direction, &current, 1) {
                Ok(coordinates) => coordinates,
                Err(_) => {
                    continue;
                }
            };

            if !visited.contains(&neighbor)
                && !maze.grid[current.0][current.1].walls.contains(direction)
            {
                stack.push(neighbor);
            }
        }
    }

    visited.len() == maze.width * maze.height
}
