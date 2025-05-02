use std::collections::HashSet;

use rand::{rngs::StdRng, Rng};

use crate::{direction::Direction, environment::environment::Coordinate, maze::maze::Maze};

pub fn growing_tree_maze(
    maze: &Maze,
    mut rng: StdRng,
    chooser: &dyn Fn(&[Coordinate]) -> &Coordinate,
) -> Vec<(Coordinate, Direction)> {
    let mut active = vec![];
    let mut walls_to_break = vec![];
    let first_end = *maze.get_end_point().iter().next().unwrap();
    let mut visited = HashSet::from([first_end]);
    active.push(first_end);
    while !active.is_empty() {
        // println!("{:?}", active);
        // println!("{:?}", visited);
        let current = *chooser(&active);
        let mut new_cells = vec![];
        for dir in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let new_coordinates = match maze.move_from(&dir, &current, 1) {
                Ok(coordinates) => coordinates,
                Err(_) => {
                    continue;
                }
            };

            if visited.contains(&new_coordinates) {
                continue;
            }

            new_cells.push((dir, new_coordinates));
        }
        // println!("{:?}", active);

        if new_cells.is_empty() {
            active = active
                .into_iter()
                .filter(|coord| *coord != current)
                .collect();
            continue;
        }
        let chosen_cell = new_cells.remove(rng.random_range(0..new_cells.len()));
        walls_to_break.push((current, chosen_cell.0));
        visited.insert(chosen_cell.1);
        active.push(chosen_cell.1);
    }
    walls_to_break
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::{maze::maze::Maze, test_utils::all_tiles_reachable::all_tiles_reachable};

    use super::*;

    #[test]
    fn test_growing_tree() {
        for _ in 0..100 {
            let mut maze = Maze::new(20, 20);
            maze.set_end((maze.width / 2, maze.height / 2));
            let walls_to_break =
                growing_tree_maze(&mut maze, StdRng::from_rng(&mut rand::rng()), &|list| {
                    &list[rand::rng().random_range(0..list.len())]
                });
            maze.break_walls_for_path(walls_to_break);

            assert!(all_tiles_reachable(&maze));
        }
    }
}
