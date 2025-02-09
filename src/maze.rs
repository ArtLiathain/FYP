pub mod maze {
    use std::{collections::HashSet, fmt, fs::File, io::Read};
    use serde::{Serialize, Deserialize};
    use pyo3::{pyclass, pymethods};
    use rand::Rng;

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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Maze {
        pub width: usize,
        pub height: usize,
        pub grid: Vec<Vec<Cell>>,
        pub path: HashSet<(usize, usize)>,
        pub visited: HashSet<(usize, usize)>,
        #[pyo3(get)]

        pub start: (usize, usize),
        #[pyo3(get)]
        pub end: (usize, usize),
        #[pyo3(get)]
        pub steps: usize,
        #[pyo3(get)]
        pub current_location: (usize, usize),
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
                path: HashSet::new(),
                visited: HashSet::new(),
                steps: 0,
                current_location: (0, height - 1),
            }
        }

        pub fn set_end(&mut self, cell: (usize, usize)) {
            self.end = cell;
        }
        pub fn in_bounds(&self, cell: (usize, usize)) -> bool {
            cell.0 < self.width && cell.1 < self.height
        }

        pub fn get_starting_point(&self) -> (usize, usize) {
            self.start
        }
        pub fn get_end_point(&self) -> (usize, usize) {
            self.end
        }

        pub fn set_starting_point(
            &mut self,
            coordinates: (usize, usize),
            delete_wall: Option<&Direction>,
        ) {
            let cell = &mut self.grid[coordinates.0][coordinates.1];
            if let Some(wall) = delete_wall {
                cell.walls.remove(wall);
            }
        }
        pub fn take_step(&mut self, amount: usize) {
            self.steps += amount;
        }

        pub fn move_from(
            &self,
            direction: &Direction,
            coordinates: &(usize, usize),
        ) -> (usize, usize) {
            match direction {
                Direction::North => (coordinates.0, coordinates.1.saturating_sub(1)),
                Direction::South => (coordinates.0, coordinates.1 + 1),
                Direction::East => (coordinates.0 + 1, coordinates.1),
                Direction::West => (coordinates.0.saturating_sub(1), coordinates.1),
            }
        }

        pub fn break_walls_for_path(&mut self, path: Vec<((usize, usize), Direction)>) {
            for i in 0..path.len() - 1 {
                let current_cell = path[i].0;
                let next_cell = self.move_from(&path[i].1, &path[i].0);
                let direction = path[i].1;
                self.grid[next_cell.0][next_cell.1]
                    .walls
                    .remove(&Direction::opposite_direction(&direction));
                self.grid[current_cell.0][current_cell.1]
                    .walls
                    .remove(&direction);
            }
        }
        pub fn break_walls_for_path_animated(
            &mut self,
            path: &Vec<((usize, usize), Direction)>,
            index: usize,
        ) {
            let current_cell = path[index].0;
            let next_cell = self.move_from(&path[index].1, &path[index].0);
            let direction = path[index].1;
            self.grid[next_cell.0][next_cell.1]
                .walls
                .remove(&Direction::opposite_direction(&direction));
            self.grid[current_cell.0][current_cell.1]
                .walls
                .remove(&direction);
        }
    }

    #[pymethods]
    impl Maze {
        pub fn move_from_current(&mut self, direction: &Direction) -> (usize, usize) {
            if self.grid[self.current_location.0][self.current_location.1]
                .walls
                .contains(direction)
            {
                return self.current_location;
            }
            self.steps += 1;
            match direction {
                Direction::North => {
                    self.current_location.1 = self.current_location.1.saturating_sub(1);
                }
                Direction::South => {
                    self.current_location.1 = self.current_location.1 + 1;
                }
                Direction::East => {
                    self.current_location.0 = self.current_location.0 + 1;
                }
                Direction::West => {
                    self.current_location.0 = self.current_location.0.saturating_sub(1);
                }
            }
            self.current_location
        }

    pub fn save_maze_to_file(maze: &Maze, filename: &str) -> PyResult<()> {
        // File operation happens synchronously, no async or threads needed
        let file = File::create(filename).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        serde_json::to_writer_pretty(file, maze).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        Ok(())
    }

        pub fn load_maze_from_file(filename: &str) -> std::io::Result<Maze> {
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let maze: Maze = serde_json::from_str(&contents)?;
            Ok(maze)
        }

        pub fn available_paths(&self) -> HashSet<Direction> {
            let walls = HashSet::from([
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ]);
            walls
                .difference(&self.grid[self.current_location.0][self.current_location.1].walls)
                .cloned()
                .collect()
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
