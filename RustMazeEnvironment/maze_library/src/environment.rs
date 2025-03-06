pub mod environment {
    use crate::maze::maze::{Direction, Maze};
    use pyo3::pyclass;
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;
    pub type Coordinate = (usize, usize);
    use crate::maze_gen::maze_gen::init_maze;

    #[cfg_attr(feature = "python", pyclass)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Environment {
        pub path_followed: Vec<Coordinate>,
        pub current_location: Coordinate,
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
        pub fn move_from_current(&mut self, direction: &Direction) -> Coordinate {
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

        pub fn to_json(&self) -> String {
            match serde_json::to_string(self) {
                Ok(string) => string,
                Err(e) => e.to_string(),
            }
        }

        pub fn from_json(json_str: &str) -> Result<Environment, serde_json::Error> {
            match serde_json::from_str(json_str) {
                Ok(environment) => Ok(environment),
                Err(e) => Err(e),
            }
        }
    }
}
