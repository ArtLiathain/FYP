use rand::{rngs::StdRng, Rng};

use crate::{direction::Direction, environment::environment::Coordinate, maze::maze::Maze};

pub fn random_wilson_maze(maze: &Maze, mut rng: StdRng) -> Vec<(Coordinate, Direction)> {
    let mut unvisited_nodes: Vec<Coordinate> = (0..maze.width)
        .flat_map(|x| (0..maze.height).map(move |y| (x, y)))
        .collect();

    let mut walls_to_break: Vec<(Coordinate, Direction)> = Vec::new();
    let mut visited_nodes = maze.end.clone();

    while !unvisited_nodes.is_empty() {
        let mut current = unvisited_nodes.remove(rng.random_range(0..unvisited_nodes.len()));
        if visited_nodes.contains(&current) {
            continue;
        }
        let mut new_path: Vec<(Coordinate, Direction)> = Vec::new();
        loop {
            let direction = Direction::random(&mut rng);
            let new_coordinates = match maze.move_from(&direction, &current, 1) {
                Ok(coordinates) => coordinates,
                Err(_) => {
                    continue;
                }
            };

            new_path.push((current, direction));

            let match_index = new_path
                .iter()
                .position(|(coordinates, _)| *coordinates == new_coordinates);

            if let Some(index) = match_index {
                new_path.truncate(index + 1);
                current = new_path.remove(new_path.len() - 1).0;
                continue;
            }

            if visited_nodes.contains(&new_coordinates)
            {
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
    fn test_wilsons() {
        for _ in 0..10 {
            let mut maze = Maze::new(20, 20);
            maze.set_end((maze.width / 2, maze.height / 2));
            let walls_to_break =
                random_wilson_maze(&mut maze, SeedableRng::from_rng(&mut rand::rng()));
            maze.break_walls_for_path(walls_to_break);

            assert!(all_tiles_reachable(&maze));
        }
    }
}
