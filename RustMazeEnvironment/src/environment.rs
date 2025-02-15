pub mod environment {
    use std::collections::HashSet;
    use pyo3::{ pyclass, pymethods, PyErr, PyResult};
    use serde::{Deserialize, Serialize};
    use crate::maze::maze::{Direction, Maze};
    pub type Coordinate = (usize, usize);
    use crate::maze_gen::maze_gen::init_maze;    


    #[pyclass]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Environment {
        pub path_followed: Vec<Coordinate>,
        #[pyo3(get)]
        pub current_location: Coordinate,
        #[pyo3(get)]
        pub maze: Maze,
        pub steps: usize,
        pub visited: HashSet<Coordinate>,
    }

    impl Environment {
        pub fn new(width: usize, height: usize) -> Environment {
            let maze = init_maze(width, height);
            Environment {
                current_location: maze.start,
                path_followed: Vec::new(),
                maze,
                steps: 0,
                visited: HashSet::new(),
            }
        }
    }

    #[pymethods]
    impl Environment {
        pub fn move_from_current(&mut self, direction: &Direction) -> Coordinate {
            if self.maze.grid[self.current_location.0][self.current_location.1]
                .walls
                .contains(direction)
            {
                return self.current_location;
            }
            self.steps += 1;
            self.path_followed.push(self.current_location);
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

        pub fn take_step(&mut self, amount: usize) {
            self.steps += amount;
        }

        pub fn to_json(&self) -> PyResult<String> {
            serde_json::to_string(self)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }

        #[staticmethod]
        pub fn from_json(json_str: &str) -> PyResult<Environment> {
            serde_json::from_str(json_str)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }

        pub fn available_paths(&self) -> HashSet<Direction> {
            let walls = HashSet::from([
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ]);
            walls
                .difference(&self.maze.grid[self.current_location.0][self.current_location.1].walls)
                .cloned()
                .collect()
        }
    }
}
