pub mod maze_gen {
    use std::collections::HashSet;

    use rand::{seq::IteratorRandom, Rng};
    use union_find::{QuickUnionUf, UnionBySize, UnionFind};

    use crate::{environment::environment::Coordinate, maze::maze::{Direction, Maze}};
    pub fn init_maze(width: usize, height: usize) -> Maze {
        let mut maze: Maze = Maze::new(width, height);
        maze.set_end((width / 2, height / 2));
        maze.set_starting_point((0, height - 1), None);
        maze
    }

    pub fn random_wilson_maze(maze: &mut Maze) -> Vec<(Coordinate, Direction)> {
        let mut unvisited_nodes: Vec<Coordinate> = (0..maze.width)
            .flat_map(|x| (0..maze.height).map(move |y| (x, y)))
            .collect();
        let mut walls_to_break: Vec<(Coordinate, Direction)> = Vec::new();
        let mut visited_nodes: HashSet<Coordinate> = HashSet::new();
        let end_coordinate = (maze.width / 2, maze.height / 2);

        let mut current = unvisited_nodes.remove(rand::thread_rng().gen_range(0..unvisited_nodes.len()));
        while !unvisited_nodes.is_empty() {
            let mut new_path: Vec<(Coordinate, Direction)> = Vec::new();
            let mut new_coordinates: Coordinate;
            loop {
                let direction = Direction::random();
                new_coordinates = maze.move_from(&direction, &current);
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
                    break;
                }
                current = new_coordinates;
            }
            walls_to_break.extend(new_path.clone());
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
            let new_coordinates = maze.move_from(&direction, &current);
            if !Maze::in_bounds(&maze, new_coordinates) || new_coordinates == current {
                continue;
            }
            path.push((current, direction));

            break;
        }
        walls_to_break.extend(path.clone());

        walls_to_break
    }

    fn unique_coordinate_index(coord: Coordinate, width: usize) -> usize {
        coord.1 * width + coord.0
    }

    pub fn random_kruzkals_maze(maze: &mut Maze) -> Vec<(Coordinate, Direction)> {
        let mut walls_to_break: Vec<(Coordinate, Direction)> = Vec::new();
        let mut edge_set: HashSet<(Coordinate, Direction)> = HashSet::new();
        let mut union_find = QuickUnionUf::<UnionBySize>::new(maze.width * maze.height);
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
            let mut rng = rand::thread_rng();
            let random_edge = match edge_set.iter().choose(&mut rng).cloned() {
                Some(edge) => edge,
                None => break,
            };
            let new_cell = maze.move_from(&random_edge.1, &random_edge.0);
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
}