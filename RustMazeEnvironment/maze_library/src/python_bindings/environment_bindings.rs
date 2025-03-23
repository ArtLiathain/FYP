use std::collections::HashMap;

use pyo3::{pyclass, pymethods, PyErr, PyResult};

use crate::{
    direction::Direction,
    environment::environment::{Coordinate, Environment},
};

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
    available_paths: HashMap<Direction, usize>,
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
    #[pyo3(get)]
    goal_dx: i32,
    #[pyo3(get)]
    goal_dy: i32,
    #[pyo3(get)]
    visited_node: bool,
}

fn calculate_manhattan_distance(pos1: Coordinate, pos2: Coordinate) -> usize {
    (pos2.0).abs_diff(pos1.0) + (pos2.1).abs_diff(pos1.1)
}

#[pymethods]
impl Environment {
    pub fn take_action(&mut self, action: Action) -> ActionResult {
        self.path_followed.push(self.current_location);
        self.visited.insert(self.current_location);
        let old_location = self.current_location;
        self.move_from_current(&action.direction);
        let mut reward = 0;
        let mut is_done = false;
        let mut is_truncated = false;
        if self.current_location == self.maze.end {
            self.path_followed.push(self.current_location);
            self.visited.insert(self.current_location);
            is_done = true;
            reward = 1000;
        }
        if calculate_manhattan_distance(self.current_location, self.maze.end)
            < calculate_manhattan_distance(old_location, self.maze.end)
            && !self.visited.contains(&self.current_location)
        {
            reward += 10;
        }

        if self.visited.contains(&self.current_location)
            && self.steps > self.maze.width * self.maze.height
        {
            reward -= 3;
        }

        if !self.visited.contains(&self.current_location)
        {
            reward += 2;
        }

        if self.steps > self.maze.width * self.maze.height * 3 {
            is_truncated = true;
        }
        let info = Info {
            manhattan_distance: calculate_manhattan_distance(self.current_location, self.maze.end),
            goal_dx: self.maze.end.0 as i32 - self.current_location.0 as i32,
            goal_dy: self.maze.end.1 as i32 - self.current_location.1 as i32,
            visited_node: self.visited.contains(&self.current_location),
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
                goal_dx: self.maze.end.0 as i32 - self.current_location.0 as i32,
                goal_dy: self.maze.end.1 as i32 - self.current_location.1 as i32,
                visited_node: self.visited.contains(&self.current_location),
            },
        }
    }

    pub fn to_json_python(&self) -> PyResult<String> {
        match serde_json::to_string(self) {
            Ok(json) => Ok(json),
            Err(e) => {
                println!("Serialization error: {}", e); // Log the error
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                ))
            }
        }
    }

    #[staticmethod]
    pub fn from_json_python(json_str: &str) -> PyResult<Environment> {
        serde_json::from_str(json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}
