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
    reward: f64,
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
    visited_paths: HashMap<Direction, f64>,
    #[pyo3(get)]
    current_location: Coordinate,
    #[pyo3(get)]
    end_node: Coordinate,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Info {
    #[pyo3(get)]
    previous_direction: usize,
    #[pyo3(get)]
    manhattan_distance: usize,
    #[pyo3(get)]
    goal_dx: i32,
    #[pyo3(get)]
    goal_dy: i32,
    #[pyo3(get)]
    visited_amount: f64,
    #[pyo3(get)]
    previous_node: Coordinate,
}

impl Info {
    pub fn create_info(env: &Environment, old_location: Coordinate) -> Info {
        Info {
            previous_direction: env.previous_direction.unwrap_or(Direction::North) as usize,
            manhattan_distance: calculate_manhattan_distance(env.current_location, (0,0)),
            goal_dx: (0,0).0 as i32 - env.current_location.0 as i32,
            goal_dy: (0,0).1 as i32 - env.current_location.1 as i32,
            visited_amount: 1.0
                - *env.visited.get(&env.current_location).unwrap_or(&0) as f64 / 5.0,
            previous_node: old_location,
        }
    }
}

fn calculate_manhattan_distance(pos1: Coordinate, pos2: Coordinate) -> usize {
    (pos2.0).abs_diff(pos1.0) + (pos2.1).abs_diff(pos1.1)
}

impl Environment {
    fn calculate_reward_for_solving(
        &self,
        old_location: Coordinate,
        old_direction: Option<Direction>,
    ) -> (bool, bool, f64) {
        let mut is_done = false;
        let mut is_truncated = false;
        let mut reward = 0.0;

        //For turnin penalties
        if old_direction.is_some() {
            //This is actually the new direction due to it being caclulated after moving
            let difference = self
                .previous_direction
                .unwrap()
                .turn_amount(&old_direction.unwrap());
            reward -= difference as f64 * 1.0;
        }

        let number_visits = *self.visited.get(&self.current_location).unwrap_or(&0);
        if number_visits > 0 {
            reward -= f64::min(0.3, number_visits as f64 * 0.1);
        } else {
            reward += 0.5;
        }

        if calculate_manhattan_distance(self.current_location, (0,0))
            < calculate_manhattan_distance(old_location, (0,0))
            && number_visits < 3
        {
            reward += 0.5;
        }

        if self.path_followed.len() >= 4 {
            if self.path_followed[self.path_followed.len() - 1]
                == self.path_followed[self.path_followed.len() - 3]
            {
                reward -= 0.7; // Penalty for oscillating motion
            }
        }

        //Running into a wall essentially
        if self.current_location == old_location {
            reward -= 1.0;
        }

        if number_visits > 5 {
            is_truncated = true;
            reward -= 10.0;
        }

        if self.current_location == (0,0) {
            is_done = true;
            reward += 50.0;
        }

        (is_done, is_truncated, reward)
    }

    fn calculate_visited_paths(&self) -> HashMap<Direction, f64> {
        self
            .available_paths()
            .iter()
            .map(|(d, steps)| {
                (
                    *d,
                    1.0 - *self
                        .visited
                        .get(
                            &self
                                .maze
                                .move_from(&*d, &self.current_location, *steps)
                                .unwrap(),
                        )
                        .unwrap_or(&0) as f64
                        / 5.0,
                )
            })
            .collect()
    }
}

#[pymethods]
impl Environment {
    pub fn take_action(&mut self, action: Action) -> ActionResult {
        let old_location = self.current_location;
        let old_direction = self.previous_direction;
        self.move_from_current(&action.direction);
        let (is_done, is_truncated, reward) =
            self.calculate_reward_for_solving(old_location, old_direction);
        if is_done {
            self.path_followed.push(self.current_location);
            *self.visited.entry(self.current_location).or_insert(0) += 1;
        }

        let observation = Observation {
            available_paths: self.available_paths(),
            visited_paths: self.calculate_visited_paths(),
            current_location: self.current_location,
            end_node: (0,0),
        };
        ActionResult {
            observation,
            reward,
            is_done,
            is_truncated,
            info: Info::create_info(&self, old_location),
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
                visited_paths: self.calculate_visited_paths(),
                current_location: self.current_location,
                end_node: (0,0),
            },
            is_done: false,
            is_truncated: false,
            reward: 0.0,
            info: Info::create_info(&self, (0, 0)),
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
