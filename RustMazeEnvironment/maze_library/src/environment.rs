pub mod environment {
    use crate::{direction::Direction, maze::maze::Maze};
    use log::error;
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};
    pub type Coordinate = (usize, usize);
    use crate::maze_gen::maze_gen::init_maze;

    #[cfg_attr(feature = "python", pyo3::pyclass)]
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Environment {
        pub path_followed: Vec<Coordinate>,
        pub current_location: Coordinate,
        pub maze: Maze,
        pub steps: usize,
        pub visited: HashSet<Coordinate>,
        #[serde(skip)]
        pub weighted_graph: HashMap<Coordinate, HashMap<Direction, usize>>,
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
                weighted_graph: HashMap::new(),
            }
        }
    }

    impl Environment {
        pub fn move_from_current(&mut self, direction: &Direction) {
            let steps = self
                .weighted_graph
                .get(&self.current_location)
                .and_then(|inner_map| inner_map.get(direction))
                .copied() // Extracts the value as usize instead of &usize
                .unwrap_or(0);
            self.steps += steps;
            match self
                .maze
                .move_from(direction, &self.current_location, Some(steps))
            {
                Ok(new_loc) => self.current_location = new_loc,
                Err(_e) => {
                    error!("MOVE ERROR WITH WEIGHTED GRAPH");
                }
            }
        }

        pub fn available_paths(&self) -> HashMap<Direction, usize> {
            self.weighted_graph
                .get(&self.current_location) // Option<&HashMap<Direction, usize>>
                .map(|paths| paths.clone()) // Clone the inner HashMap
                .unwrap_or_default()
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
