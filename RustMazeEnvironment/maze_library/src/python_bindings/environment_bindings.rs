use pyo3::{pyclass, pymethods, PyErr, PyResult};

use crate::{
    constants::constants::NUMBER_OF_INPUT_FEATURES,
    direction::Direction,
    environment::environment::{Coordinate, Environment},
    maze::maze::Maze,
    maze_gen::maze_gen_handler::{select_maze_algorithm, MazeType},
};

use super::environment_observations::{calculate_manhattan_distance, Observation};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Action {
    pub direction: usize,
    pub run: usize,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ActionResult {
    #[pyo3(get)]
    observation: Observation,
    #[pyo3(get)]
    reward: f32,
    #[pyo3(get)]
    is_done: bool,
    #[pyo3(get)]
    is_truncated: bool,
}

impl ActionResult {
    pub fn flatten_and_scale(&self, env: &Environment) -> (Vec<f32>, f32, bool, bool) {
        (
            self.observation.flatten_and_scale_observation(&env),
            self.reward,
            self.is_done,
            self.is_truncated,
        )
    }
}

#[pymethods]
impl Environment {
    pub fn take_action(&mut self, action: Action) -> (Vec<f32>, f32, bool, bool) {
        let old_location = self.current_location;
        let dir = Direction::from(action.direction);
        let old_direction = self.previous_direction;
        let steps_taken = self.move_from_current(&dir, action.run);
        if steps_taken == 0 {
            self.steps += 1
        }
        let (is_done, is_truncated, reward);
        if action.run > self.config.python_config.mini_explore_runs_per_episode {
            (is_done, is_truncated, reward) =
                self.calculate_reward_for_solving(old_location, old_direction);
        } else {
            (is_done, is_truncated, reward) = self.calculate_reward_for_exploring(old_direction);
        }

        ActionResult {
            observation: Observation::new(&self, old_location),
            reward: (reward * (steps_taken as f32).max(1.0)),
            is_done,
            is_truncated,
        }
        .flatten_and_scale(&self)
    }

    pub fn reset(&mut self) -> Vec<f32> {
        self.visited.clear();
        self.total_steps += self.steps;
        self.steps = 0;
        self.current_location = self.maze.start;
        Observation::new(&self, self.maze.get_starting_point()).flatten_and_scale_observation(&self)
    }
    pub fn smart_reset(&mut self, run: usize) -> Vec<f32> {
        if self.config.python_config.mini_explore_runs_per_episode < run {
            self.visited.clear();
        }
        self.total_steps += self.steps;
        self.steps = 0;
        self.current_location = self.maze.start;
        Observation::new(&self, self.maze.get_starting_point()).flatten_and_scale_observation(&self)
    }

    pub fn input_shape(&self) -> usize {
        NUMBER_OF_INPUT_FEATURES
    }
    pub fn output_shape(&self) -> usize {
        4
    }

    pub fn reset_and_regenerate(&mut self) -> Vec<f32> {
        let mut maze = Maze::init_maze(self.maze.width, self.maze.height);
        let walls = select_maze_algorithm(&maze, None, &MazeType::BinaryTree);
        maze.break_walls_for_path(walls);
        self.weighted_graph = maze.convert_to_weighted_graph(None, true);
        self.maze = maze;
        self.path_followed.clear();
        self.current_location = self.maze.start;
        self.visited.clear();
        self.overall_visited.clear();
        self.steps = 0;
        self.total_steps = 0;
        Observation::new(&self, self.maze.get_starting_point()).flatten_and_scale_observation(&self)
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

impl Environment {
    fn calculate_reward_for_solving(
        &self,
        old_location: Coordinate,
        old_direction: Option<Direction>,
    ) -> (bool, bool, f32) {
        let mut is_done = false;
        let mut is_truncated = false;
        let mut reward = 0.0;
        let end = self.maze.get_perfect_end_centre();
        if old_direction.is_some() {
            //This is actually the new direction due to it being caclulated after moving
            let difference = self
                .previous_direction
                .unwrap()
                .turn_amount(&old_direction.unwrap());
            reward -= difference as f32 * 0.1;
        }

        let number_visits = *self.visited.get(&self.current_location).unwrap_or(&1) - 1;
        if number_visits > 0 {
            reward -= f32::min(0.5, number_visits as f32 * 0.15);
        }

        if calculate_manhattan_distance(self.current_location, end)
            < calculate_manhattan_distance(old_location, end)
        {
            reward += 0.5;
        }
        //Running into a wall essentially
        if self.current_location == old_location {
            reward -= 0.5;
        }

        if self.steps > self.config.python_config.exploration_steps {
            reward -= 5.0;
            is_truncated = true;
        }

        if self.maze.end.contains(&self.current_location) {
            is_done = true;
            reward += 30.0 + 500.0 / self.steps as f32;
        }

        (is_done, is_truncated, reward)
    }

    fn calculate_reward_for_exploring(
        &self,
        old_direction: Option<Direction>,
    ) -> (bool, bool, f32) {
        let mut is_truncated = false;
        let mut reward = 0.1;
        if old_direction.is_some() {
            //This is actually the new direction due to it being caclulated after moving
            let difference = self
                .previous_direction
                .unwrap()
                .turn_amount(&old_direction.unwrap());
            reward -= difference as f32 * 0.1;
        }

        let number_visits = *self
            .overall_visited
            .get(&self.current_location)
            .unwrap_or(&1) - 1;
        let recent_number_visits = *self
            .visited
            .get(&self.current_location)
            .unwrap_or(&1) - 1;
        if number_visits == 0 {
            reward += 0.15
             * self.overall_visited.len() as f32;
            }
            if recent_number_visits > 0 {
                reward -= f32::min(1.5, number_visits as f32 * 0.3);
            }
            else {
                reward = 0.0
            }

        if self.steps > self.config.python_config.exploration_steps * 2 {
            is_truncated = true;
        }
        
        (false, is_truncated, reward)
    }
}
