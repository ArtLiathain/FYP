use std::collections::HashSet;

use rand::Rng;

use crate::{direction::Direction, environment::environment::Coordinate, maze::maze::Maze};

pub fn growing_tree(maze: &Maze, chooser: &dyn Fn(&[Coordinate]) -> &Coordinate) -> Vec<(Coordinate, Direction)> {
    let mut active = vec![];
    let mut walls_to_break = vec![];
    let first_end = *maze.get_end_point().iter().next().unwrap();
    let mut visited = HashSet::from([first_end]);
    let mut end_visited = false;
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
            
            let r1 = maze.end.contains(&current);
            let nc = maze.end.contains(&new_coordinates); 
            if visited.contains(&new_coordinates) || ( ((r1 && !nc) || (nc && !r1)) && end_visited){
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
        let chosen_cell = new_cells.remove(rand::rng().random_range(0..new_cells.len()));
        let r1 = maze.end.contains(&current);
        let nc = maze.end.contains(&chosen_cell.1);
        if (r1 && !nc) || (nc && !r1) {
            end_visited = true;
        }
        walls_to_break.push((current, chosen_cell.0));
        visited.insert(chosen_cell.1);
        active.push(chosen_cell.1);
    }
    walls_to_break
}
