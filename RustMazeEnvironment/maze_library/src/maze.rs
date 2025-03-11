pub mod maze {
    use serde::{Deserialize, Serialize};
    use std::{
        collections::{HashMap, HashSet},usize
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
        pub end: Coordinate,
    }

    impl Maze {
        pub fn new(width: usize, height: usize) -> Self {
            Maze {
                width,
                height,
                start: (0, height - 1),
                end: (width / 2, height / 2),
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

        pub fn set_end(&mut self, cell: Coordinate) {
            self.end = cell;
        }
        pub fn in_bounds(&self, cell: (i32, i32)) -> bool {
            cell.0 < self.width as i32 && cell.1 < self.height as i32 && cell.0 > -1 && cell.1 > -1
        }

        pub fn get_starting_point(&self) -> Coordinate {
            self.start
        }
        pub fn get_end_point(&self) -> Coordinate {
            self.end
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

        pub fn move_from_with_walls(&self,
            direction: &Direction,
            coordinates: &Coordinate) -> Result<Coordinate, MoveError> {
            if self.grid[coordinates.0][coordinates.1].walls.contains(direction) {
                return Err(MoveError::OutOfBounds);
            }
            self.move_from(direction, coordinates, None)
            
        }

        pub fn move_from(
            &self,
            direction: &Direction,
            coordinates: &Coordinate,
            steps : Option<usize>
        ) -> Result<Coordinate, MoveError> {
            let (x,  y)  = (coordinates.0 as i32, coordinates.1 as i32);
            let steps_to_take = steps.unwrap_or(1) as i32;
            let new_coordinates = match direction {
                Direction::North => {
                    (x, y - steps_to_take)
                }
                Direction::South => {
                    (x, y + steps_to_take)
                }
                Direction::East => {
                    (x + steps_to_take, y)
                }
                Direction::West => {
                    (x - steps_to_take, y)
                }
            };
            if !self.in_bounds(new_coordinates) {
               return Err(MoveError::OutOfBounds);
            }
            Ok((new_coordinates.0 as usize, new_coordinates.1 as usize))
        }

        pub fn break_walls_for_path(&mut self, path: Vec<(Coordinate, Direction)>) {
            for i in 0..path.len() {
                self.break_wall_for_path(&path, i);
            }
        }
        pub fn break_wall_for_path(&mut self, path: &Vec<(Coordinate, Direction)>, index: usize) {
            let current_cell = path[index].0;
            let next_cell = match self.move_from(&path[index].1, &path[index].0, None) {
                Ok(coordinates) => {coordinates},
                Err(_) => {return;}
            };
            let direction = path[index].1;
            self.grid[next_cell.0][next_cell.1]
                .walls
                .remove(&Direction::opposite_direction(&direction));
            self.grid[current_cell.0][current_cell.1]
                .walls
                .remove(&direction);
        }

        fn follow_path(&self, coordinates: &Coordinate, direction: &Direction, decision_nodes: &HashSet<Coordinate>) -> usize {
            let mut steps = 0;
            let mut current = *coordinates;
            loop {
                current = match self.move_from_with_walls(direction, &current) {
                    Ok(coordinates) => {coordinates},
                    Err(_) => {return steps}
                };
                steps+=1;
                if decision_nodes.contains(&current) {
                    break;
                }
            }
            steps
        }

        pub fn convert_to_weighted_graph(
            &self,
        ) -> HashMap<Coordinate, HashMap<Direction, usize>> {
            let mut decision_nodes: HashMap<Coordinate, HashMap<Direction, usize>> =
                HashMap::new();
            let mut decision_set: HashSet<Coordinate> = HashSet::new();

            decision_nodes.insert(self.start, HashMap::new());
            decision_nodes.insert(self.end, HashMap::new());
            decision_set.insert(self.start);
            decision_set.insert(self.end);

            for row in 0..self.height {
                for column in 0..self.width {
                    let cell = &self.grid[row][column];
                    let walls: Vec<&Direction> = cell.walls.iter().collect();
                    if walls.len() <= 1 {
                        decision_nodes.insert((cell.x, cell.y), HashMap::new());
                        decision_set.insert((cell.x, cell.y));
                    }
                    if walls.len() == 2 && *walls[0] != walls[1].opposite_direction() {
                        decision_nodes.insert((cell.x, cell.y), HashMap::new());
                        decision_set.insert((cell.x, cell.y));
                    }
                }
            }
            let directions = vec!(Direction::North,Direction::South,Direction::East,Direction::West);
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
