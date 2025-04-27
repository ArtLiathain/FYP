use std::collections::HashSet;

use rand::{rngs::StdRng, seq::IteratorRandom};
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

use crate::{direction::Direction, environment::environment::Coordinate, maze::maze::Maze};

fn unique_coordinate_index(coord: Coordinate, width: usize) -> usize {
    coord.1 * width + coord.0
}

pub fn random_kruzkals_maze(maze: &Maze, mut rng: StdRng) -> Vec<(Coordinate, Direction)> {
    let mut walls_to_break: Vec<(Coordinate, Direction)> = Vec::new();
    let mut edge_set: HashSet<(Coordinate, Direction)> = HashSet::new();
    let mut union_find = QuickUnionUf::<UnionBySize>::new(maze.width * maze.height);
    let mut end_visited = false;

    //Put all edges into a burlap sack
    for x in 0..maze.width {
        for y in 0..maze.height {
            if x + 1 < maze.width {
                edge_set.insert(((x, y), Direction::East));
            }
            if y + 1 < maze.height {
                edge_set.insert(((x, y), Direction::South));
            }
        }
    }
    while !edge_set.is_empty() {
        let random_edge = match edge_set.iter().choose(&mut rng).cloned() {
            Some(edge) => edge,
            None => break,
        };
        let new_cell = match maze.move_from(&random_edge.1, &random_edge.0, 1) {
            Ok(coordinates) => coordinates,
            Err(_) => {
                continue;
            }
        };
        let r1 = maze.end.contains(&random_edge.0);
        let nc = maze.end.contains(&new_cell);
        if (r1 && !nc) || (nc && !r1) {
            if end_visited {
                edge_set.remove(&random_edge);
                continue;
            }
            end_visited = true;
        }
        let cell_union_set = unique_coordinate_index(random_edge.0, maze.width);
        let new_cell_union_set = unique_coordinate_index(new_cell, maze.width);
        if union_find.find(cell_union_set) == union_find.find(new_cell_union_set) {
            edge_set.remove(&random_edge);
            continue;
        }
        union_find.union(cell_union_set, new_cell_union_set);
        walls_to_break.push(random_edge);
        edge_set.remove(&random_edge);
    }

    walls_to_break
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::{maze::maze::Maze, test_utils::all_tiles_reachable::all_tiles_reachable};

    use super::*;

    #[test]
    fn test_kruskals() {
        for _ in 0..10 {
            let mut maze = Maze::new(20, 20);
            maze.set_end((maze.width / 2, maze.height / 2));
            let walls_to_break =
                random_kruzkals_maze(&mut maze, SeedableRng::from_rng(&mut rand::rng()));
            maze.break_walls_for_path(walls_to_break);
            assert!(all_tiles_reachable(&maze));
        }
    }
}
