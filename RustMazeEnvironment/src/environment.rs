pub mod environment {
    use crate::maze::maze::{Direction, Maze};
    use pyo3::{pyclass, pymethods, PyErr, PyResult};
    use serde::{Deserialize, Serialize};
    use std::{collections::HashSet, path::absolute};
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

    impl Environment {
        fn move_from_current(&mut self, direction: &Direction) -> Coordinate {
            if self.maze.grid[self.current_location.0][self.current_location.1]
                .walls
                .contains(direction)
            {
                return self.current_location;
            }
            self.steps += 1;
            self.path_followed.push(self.current_location);
            self.visited.insert(self.current_location);
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

        fn available_paths(&self) -> HashSet<Direction> {
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

    #[pyclass]
    #[derive(Debug, Clone)]
    pub struct Action {
        pub direction: Direction,
    }

    #[pyclass]
    #[derive(Debug, Clone)]
    pub struct ActionResult {
        #[pyo3(get)]
        observation: Observation,
        #[pyo3(get)]
        reward: i64,
        #[pyo3(get)]
        is_done: bool,
        #[pyo3(get)]
        is_truncated: bool,
        #[pyo3(get)]
        info: Info,
    }

    #[pyclass]
    #[derive(Debug, Clone)]
    pub struct Observation {
        #[pyo3(get)]
        available_paths: HashSet<Direction>,
        #[pyo3(get)]
        current_location: Coordinate,
        #[pyo3(get)]
        end_node: Coordinate,
    }

    #[pyclass]
    #[derive(Debug, Clone)]
    pub struct Info {
        #[pyo3(get)]
        manhattan_distance: usize,
    }

    fn calculate_manhattan_distance(pos1: Coordinate, pos2: Coordinate) -> usize {
        (pos2.0).abs_diff(pos1.0) + (pos2.1).abs_diff(pos1.1)
    }

    #[pymethods]
    impl Environment {
        pub fn take_action(&mut self, action: Action) -> ActionResult {
            let _ = &self.move_from_current(&action.direction);
            let mut reward = 0;
            let mut is_done = false;
            let mut is_truncated = false;
            if self.current_location == self.maze.end {
                reward += 100;
                is_done = true;
            }
            if self.visited.contains(&self.current_location) {
                reward -= 1;
            }
            if self.steps > self.maze.width * self.maze.height * 3 {
                is_truncated = true;
            }
            let info = Info {
                manhattan_distance: calculate_manhattan_distance(
                    self.current_location,
                    self.maze.end,
                ),
            };
            let observation = Observation {
                available_paths: self.available_paths(),
                current_location: self.current_location,
                end_node: self.maze.end,
            };
            ActionResult {
                observation,
                reward,
                is_done,
                is_truncated,
                info,
            }
        }

        pub fn reset(&mut self) -> ActionResult {
            self.path_followed.clear();
            self.current_location = self.maze.start;
            self.visited.clear();
            self.steps = 0;
            ActionResult {
                observation: Observation {
                    available_paths: self.available_paths(),
                    current_location: self.current_location,
                    end_node: self.maze.end,
                },
                is_done: false,
                is_truncated: false,
                reward: 0,

                info: Info {
                    manhattan_distance: calculate_manhattan_distance(
                        self.current_location,
                        self.maze.end,
                    ),
                },
            }
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
    }
}
