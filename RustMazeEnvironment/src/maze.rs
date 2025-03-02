pub mod maze {
    use pyo3::{pyclass, pymethods};
    use rand::Rng;
    use serde::{de::value::MapDeserializer, Deserialize, Serialize};
    use std::{
        collections::{HashMap, HashSet}, fmt, usize
    };

    use crate::environment::environment::Coordinate;


    #[derive(Debug, Clone)]
    pub enum MoveError {
        OutOfBounds,
        InvalidDirection,
    }

    #[pyclass]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Cell {
        #[pyo3(get, set)]
        pub x: usize,
        #[pyo3(get, set)]
        pub y: usize,
        #[pyo3(get)]
        pub walls: HashSet<Direction>,
    }

    #[pyclass]
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Maze {
        pub width: usize,
        pub height: usize,
        pub grid: Vec<Vec<Cell>>,
        #[pyo3(get)]
        pub start: Coordinate,
        #[pyo3(get)]
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

        pub fn move_from(
            &self,
            direction: &Direction,
            coordinates: &Coordinate,
        ) -> Result<Coordinate, MoveError> {
            let (x,  y)  = (coordinates.0 as i32, coordinates.1 as i32);

            let new_coordinates = match direction {
                Direction::North => {
                    (x, y - 1)
                }
                Direction::South => {
                    (x, y + 1)
                }
                Direction::East => {
                    (x + 1, y)
                }
                Direction::West => {
                    (x - 1, y)
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
            let next_cell = match self.move_from(&path[index].1, &path[index].0) {
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

        fn follow_path(maze: &Maze, coordinates: &Coordinate, direction: &Direction) -> usize {
            let mut steps = 0;
            let mut current = coordinates;
            loop {}
        }

        pub fn convert_to_weighted_graph(
            maze: &Maze,
        ) -> HashMap<Coordinate, (Coordinate, Direction, usize)> {
            let mut decision_nodes: HashMap<Coordinate, (Coordinate, Direction, usize)> =
                HashMap::new();
            for row in 0..maze.height {
                for column in 0..maze.height {
                    let cell = &maze.grid[row][column];
                    let walls: Vec<&Direction> = cell.walls.iter().collect();
                    if walls.len() <= 1 {
                        decision_nodes.insert((cell.x, cell.y), ((0, 0), Direction::North, 0));
                    }
                    if walls.len() == 2 && *walls[0] != walls[1].opposite_direction() {
                        decision_nodes.insert((cell.x, cell.y), ((0, 0), Direction::North, 0));
                    }
                }
            }

            HashMap::new()
        }
    }

    #[repr(usize)]
    #[pyclass(eq, eq_int)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Direction {
        North = 0,
        South = 1,
        East = 2,
        West = 3,
    }

    impl fmt::Display for Direction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let direction_str = match self {
                Direction::North => "North",
                Direction::South => "South",
                Direction::East => "East",
                Direction::West => "West",
            };
            write!(f, "{}", direction_str)
        }
    }
    #[pymethods]
    impl Direction {
        pub fn __hash__(&self) -> u64 {
            *self as u64
        }

        pub fn opposite_direction(&self) -> Direction {
            match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            }
        }
    }

    impl Direction {
        pub fn random() -> Direction {
            match rand::thread_rng().gen_range(0..4) {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                _ => Direction::West,
            }
        }
    }
}
