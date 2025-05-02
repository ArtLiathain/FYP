use std::collections::HashSet;

use rand::{rngs::StdRng, Rng};

use crate::{direction::Direction, environment::environment::Coordinate, maze::maze::Maze};

pub fn random_binary_maze(maze: &Maze, mut rng: StdRng) -> Vec<(Coordinate, Direction)> {
    let mut unvisited_nodes: Vec<Coordinate> = (0..maze.width)
        .flat_map(|x| (0..maze.height).map(move |y| (x, y)))
        .collect();

    let mut walls_to_break: Vec<(Coordinate, Direction)> = vec![];
    let mut visited_nodes = HashSet::from([(maze.width - 1, 0)]);
    //top right node is considered visited
    let directions = [Direction::North, Direction::East];
    while !unvisited_nodes.is_empty() {
        let mut current = unvisited_nodes.remove(rng.random_range(0..unvisited_nodes.len()));
        if visited_nodes.contains(&current) {
            continue;
        }
        let mut new_path: Vec<(Coordinate, Direction)> = Vec::new();
        loop {
            let direction = directions[rng.random_range(0..2)];

            let new_coordinates = match maze.move_from(&direction, &current, 1) {
                Ok(coordinates) => coordinates,
                Err(_) => {
                    continue;
                }
            };

            new_path.push((current, direction));

            if visited_nodes.contains(&new_coordinates) {
                break;
            }
            current = new_coordinates;
        }
        for (coords, _) in new_path.iter() {
            visited_nodes.insert(*coords);
            unvisited_nodes.retain(|&coord| coord != *coords);
        }
        walls_to_break.extend(new_path);
    }

    walls_to_break
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::{maze::maze::Maze, test_utils::all_tiles_reachable::all_tiles_reachable};

    use super::*;

    #[test]
    fn test_binary_tree() {
        for _ in 0..10 {
            let mut maze = Maze::new(10, 10);
            maze.set_end((maze.width / 2, maze.height / 2));
            let walls_to_break =
                random_binary_maze(&mut maze, SeedableRng::from_rng(&mut rand::rng()));
            maze.break_walls_for_path(walls_to_break);

            assert!(all_tiles_reachable(&maze));
        }
    }
}
