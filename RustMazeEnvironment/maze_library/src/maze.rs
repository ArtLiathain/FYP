pub mod maze {
    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use std::{
        collections::{HashMap, HashSet},
        usize,
    };

    use crate::{direction::Direction, environment::environment::Coordinate};

    #[derive(Debug, Clone)]
    pub enum MoveError {
        OutOfBounds,
        InvalidDirection,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Cell {
        pub x: usize,
        pub y: usize,
        pub walls: HashSet<Direction>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Maze {
        pub width: usize,
        pub height: usize,
        pub grid: Vec<Vec<Cell>>,
        pub start: Coordinate,
        pub end: HashSet<Coordinate>,
    }

    pub fn directional_movement(
        direction: &Direction,
        coordinates: &Coordinate,
        steps: usize,
    ) -> (i32, i32) {
        let i32steps = steps as i32;
        let (x, y) = (coordinates.0 as i32, coordinates.1 as i32);
        match direction {
            Direction::North => (x, y - i32steps),
            Direction::South => (x, y + i32steps),
            Direction::East => (x + i32steps, y),
            Direction::West => (x - i32steps, y),
        }
    }

    pub fn to_usize_tuple(coord: (i32, i32)) -> (usize, usize) {
        (coord.0 as usize, coord.1 as usize)
    }

    impl Maze {
        pub fn new(width: usize, height: usize) -> Self {
            Maze {
                width,
                height,
                start: (0, height - 1),
                end: HashSet::new(),
                grid: (0..width)
                    .map(|x| {
                        (0..height)
                            .map(move |y| Cell {
                                x,
                                y,
                                walls: HashSet::from([
                                    Direction::North,
                                    Direction::South,
                                    Direction::East,
                                    Direction::West,
                                ]),
                            })
                            .collect::<Vec<Cell>>()
                    })
                    .collect::<Vec<Vec<Cell>>>(),
            }
        }

        pub fn init_maze(width: usize, height: usize) -> Maze {
            let mut maze: Maze = Maze::new(width, height);
            maze.set_end((width / 2, height / 2));
            maze.set_starting_point((0, height - 1), None);
            maze
        }

        pub fn set_end(&mut self, cell: Coordinate) {
            let end_set = [
                cell,
                (cell.0 - 1, cell.1),
                (cell.0, cell.1 - 1),
                (cell.0 - 1, cell.1 - 1),
            ]
            .into_iter()
            .collect();
            self.end = end_set;
        }
        pub fn in_bounds(&self, cell: (i32, i32)) -> bool {
            cell.0 < self.width as i32 && cell.1 < self.height as i32 && cell.0 > -1 && cell.1 > -1
        }

        pub fn get_starting_point(&self) -> Coordinate {
            self.start
        }
        pub fn get_end_point(&self) -> &HashSet<Coordinate> {
            &self.end
        }

        pub fn set_starting_point(
            &mut self,
            coordinates: Coordinate,
            delete_wall: Option<&Direction>,
        ) {
            let cell = &mut self.grid[coordinates.0][coordinates.1];
            if let Some(wall) = delete_wall {
                cell.walls.remove(wall);
            }
        }

        pub fn move_from_with_walls(
            &self,
            direction: &Direction,
            coordinates: &Coordinate,
        ) -> Result<Coordinate, MoveError> {
            if self.grid[coordinates.0][coordinates.1]
                .walls
                .contains(direction)
            {
                return Err(MoveError::OutOfBounds);
            }
            self.move_from(direction, coordinates, 1)
        }

        pub fn move_from(
            &self,
            direction: &Direction,
            coordinates: &Coordinate,
            steps: usize,
        ) -> Result<Coordinate, MoveError> {
            let new_coordinates = directional_movement(direction, coordinates, steps);
            if !self.in_bounds(new_coordinates) {
                return Err(MoveError::OutOfBounds);
            }
            Ok((new_coordinates.0 as usize, new_coordinates.1 as usize))
        }

        pub fn break_walls_for_path(&mut self, path: Vec<(Coordinate, Direction)>) {
            for i in 0..path.len() {
                self.break_wall_for_path(&path, i);
            }
            self.break_end_walls();
        }

        pub fn break_end_walls(&mut self) {
            for &(x, y) in self.end.iter() {
                let current = (x, y);
    
                // Check each cardinal direction
                for dir in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    if let Ok(neighbor) = self.move_from(&dir, &current, 1) {
                        if self.end.contains(&neighbor) {
                            // Remove the wall between current and neighbor
                            self.grid[current.0][current.1].walls.remove(&dir);
                            self.grid[neighbor.0][neighbor.1]
                                .walls
                                .remove(&Direction::opposite_direction(&dir));
                        }
                    }
                }
            }
        }

        pub fn break_wall_for_path(&mut self, path: &Vec<(Coordinate, Direction)>, index: usize) {
            let current_cell = path[index].0;
            let next_cell = match self.move_from(&path[index].1, &path[index].0, 1) {
                Ok(coordinates) => coordinates,
                Err(_) => {
                    return;
                }
            };
            let direction = path[index].1;
            self.grid[next_cell.0][next_cell.1]
                .walls
                .remove(&Direction::opposite_direction(&direction));
            self.grid[current_cell.0][current_cell.1]
                .walls
                .remove(&direction);
        }

        fn follow_path(
            &self,
            coordinates: &Coordinate,
            direction: &Direction,
            decision_nodes: &HashSet<Coordinate>,
        ) -> usize {
            let mut steps = 0;
            let mut current = *coordinates;
            loop {
                current = match self.move_from_with_walls(direction, &current) {
                    Ok(coordinates) => coordinates,
                    Err(_) => return steps,
                };
                steps += 1;
                if decision_nodes.contains(&current) {
                    break;
                }
            }
            steps
        }

        pub fn break_random_walls(&mut self, amount: usize) -> Vec<(Coordinate, Direction)> {
            let mut edge_set: Vec<(Coordinate, Direction)> = Vec::new();
            let mut walls_to_break: Vec<(Coordinate, Direction)> = Vec::new();
            for x in 0..self.width {
                for y in 0..self.height {
                    let valid_directions = match (x + 1 < self.width, y + 1 < self.height) {
                        (true, true) => &vec![Direction::South, Direction::East], // Both directions
                        (false, true) => &vec![Direction::South],                 // Only South
                        (true, false) => &vec![Direction::East],                  // Only East
                        _ => &vec![],                                             // No valid moves
                    };
    
                    edge_set.extend(
                        self.grid[x][y]
                            .walls
                            .iter()
                            .filter(|dir| valid_directions.contains(dir))
                            .map(|dir| ((x, y), *dir)),
                    );
                }
            }
            for _ in 0..amount {
                walls_to_break.push(edge_set.remove(rand::rng().random_range(0..edge_set.len())));
            }
            walls_to_break
        }

        pub fn convert_to_weighted_graph(
            &self,
            visited: Option<&HashMap<Coordinate, usize>>,
        ) -> HashMap<Coordinate, HashMap<Direction, usize>> {
            let mut decision_nodes: HashMap<Coordinate, HashMap<Direction, usize>> = HashMap::new();
            let mut decision_set: HashSet<Coordinate> = HashSet::new();
            let mut visited_to_use = &HashMap::new();
            if visited.is_some() {
                visited_to_use = visited.unwrap();
            }
            decision_nodes.insert(self.start, HashMap::new());
            for point in self.end.iter() {
                decision_nodes.insert(*point, HashMap::new());
            }
            decision_set.insert(self.start);
            decision_set.extend(self.end.clone());

            for row in 0..self.height {
                for column in 0..self.width {
                    if visited.is_some() && !visited_to_use.contains_key(&(row, column)) {
                        continue;
                    }
                    let cell = &self.grid[row][column];
                    let walls: Vec<&Direction> = cell.walls.iter().collect();
                    if walls.len() <= 1 || walls.len() == 3 {
                        decision_nodes.insert((cell.x, cell.y), HashMap::new());
                        decision_set.insert((cell.x, cell.y));
                    }
                    if walls.len() == 2 && *walls[0] != walls[1].opposite_direction() {
                        decision_nodes.insert((cell.x, cell.y), HashMap::new());
                        decision_set.insert((cell.x, cell.y));
                    }
                }
            }
            let directions = vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ];
            for (coordinate, inner_map) in decision_nodes.iter_mut() {
                for direction in &directions {
                    let steps = self.follow_path(&coordinate, &direction, &decision_set);
                    if steps > 0 {
                        inner_map.insert(*direction, steps);
                    }
                }
            }

            decision_nodes
        }
    }
}
