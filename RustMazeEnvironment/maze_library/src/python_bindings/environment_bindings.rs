use std::collections::HashMap;

use pyo3::{pyclass, pymethods, PyErr, PyResult};
use serde::{Deserialize, Serialize};

use crate::{
    direction::Direction,
    environment::environment::{Coordinate, Environment},
    maze::maze::Maze,
    maze_gen::maze_gen_handler::{select_maze_algorithm, MazeType},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Action {
    pub direction: usize,
    pub run: usize,
}
#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportCard {
    #[pyo3(get)]
    pub total_steps: usize,
    #[pyo3(get)]
    pub best_run_penalty: usize,
    #[pyo3(get)]
    pub best_run_steps: usize,
    #[pyo3(get)]
    pub best_run: usize,
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

#[pyclass]
#[derive(Debug, Clone)]
pub struct Observation {
    #[pyo3(get)]
    available_paths: HashMap<Direction, usize>,
    #[pyo3(get)]
    visited_paths: HashMap<Direction, usize>,
    #[pyo3(get)]
    current_location: Coordinate,
    #[pyo3(get)]
    previous_location: Coordinate,
    #[pyo3(get)]
    goal_dxdy: (f32, f32),
    #[pyo3(get)]
    previous_direction: usize,
    #[pyo3(get)]
    manhattan_distance: f32,
    #[pyo3(get)]
    current_visited_amount: usize,
    #[pyo3(get)]
    end_node: (f32, f32),
}
impl Observation {
    pub fn new(env: &Environment, previous_location: Coordinate) -> Observation {
        let end = env.maze.get_perfect_end_centre();
        Observation {
            previous_direction: env.previous_direction.unwrap_or(Direction::North) as usize,
            manhattan_distance: calculate_manhattan_distance(env.current_location, end),
            goal_dxdy: (
                end.0 - env.current_location.0 as f32,
                end.1 - env.current_location.1 as f32,
            ),
            current_visited_amount: *env.visited.get(&env.current_location).unwrap_or(&0),
            available_paths: env.available_paths(),
            visited_paths: env.calculate_visited_paths(),
            current_location: env.current_location,
            end_node: env.maze.get_perfect_end_centre(),
            previous_location,
        }
    }

    pub fn flatten_and_scale_observation(&self, env: &Environment) -> Vec<f32> {
        let mut vec = Vec::new();
        let direction_vec = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        for dir in direction_vec.iter() {
            let path = self.available_paths.get(&dir).unwrap_or(&0);

            if [Direction::North, Direction::South].contains(&dir) {
                vec.push(*path as f32 / env.maze.height as f32);
            } else {
                vec.push(*path as f32 / env.maze.width as f32);
            }
        }

        for dir in direction_vec.iter() {
            vec.push(
                1.0 - *self.visited_paths.get(&dir).unwrap_or(&0) as f32
                    / env.config.python_config.allowed_revisits as f32,
            );
        }

        vec.push(self.previous_direction as f32);
        vec.push(self.current_location.0 as f32 / (env.maze.width as f32 - 1.0));
        vec.push(self.current_location.1 as f32 / (env.maze.height as f32 - 1.0));
        vec.push(self.end_node.0 / (env.maze.width as f32 - 1.0));
        vec.push(self.end_node.1 / (env.maze.height as f32 - 1.0));
        vec.push(self.previous_location.0 as f32 / (env.maze.width as f32 - 1.0));
        vec.push(self.previous_location.1 as f32 / (env.maze.height as f32 - 1.0));
        vec.push(self.manhattan_distance / (env.maze.width + env.maze.height) as f32);
        vec.push((self.goal_dxdy.0 / (env.maze.width as f32 / 2.0) + 1.0) / 2.0);
        vec.push((self.goal_dxdy.1 / (env.maze.height as f32 / 2.0) + 1.0) / 2.0);
        vec.push(
            1.0 - self.current_visited_amount as f32
                / env.config.python_config.allowed_revisits as f32,
        );
        vec
    }
}

fn calculate_manhattan_distance(pos1: Coordinate, pos2: (f32, f32)) -> f32 {
    (pos1.0 as f32 - pos2.0).abs() + (pos1.1 as f32 - pos2.1).abs()
}

impl Environment {
    pub fn generate_report_card(&self) -> ReportCard {
        let (mut best_run_steps, mut best_run_penalty, mut best_run) = (usize::MAX, 0, 0);
        for i in 0..self.get_current_run() {
            let (total_run_steps, total_run_penalty) = self.calculate_run_score(i);
            if total_run_steps + total_run_penalty < best_run_steps + best_run_penalty {
                (best_run_steps, best_run_penalty, best_run) =
                    (total_run_steps, total_run_penalty, i)
            }
        }
        ReportCard {
            total_steps: self.steps,
            best_run_steps,
            best_run_penalty,
            best_run,
        }
    }

    fn calculate_reward_for_solving(
        &self,
        old_location: Coordinate,
        old_direction: Option<Direction>,
    ) -> (bool, bool, f32) {
        let mut is_done = false;
        let mut is_truncated = false;
        let mut reward = -0.05;
        let end = self.maze.get_perfect_end_centre();
        if old_direction.is_some() {
            //This is actually the new direction due to it being caclulated after moving
            let difference = self
                .previous_direction
                .unwrap()
                .turn_amount(&old_direction.unwrap());
            reward -= difference as f32 * 0.25;
        }

        let number_visits = *self.visited.get(&self.current_location).unwrap_or(&0);
        if number_visits > 0 {
            reward -= f32::min(0.5, number_visits as f32 * 0.15);
        } else {
            reward += 1.0;
        }

        if calculate_manhattan_distance(self.current_location, end)
            < calculate_manhattan_distance(old_location, end)
            && number_visits < 2
        {
            reward += 1.0;
        }

        //Running into a wall essentially
        if self.current_location == old_location {
            reward -= 3.0;
        }

        if number_visits > self.config.python_config.allowed_revisits {
            is_truncated = true;
            reward -= 10.0;
        }

        if self.maze.end.contains(&self.current_location) {
            is_done = true;
            reward += 30.0 + 500.0 / self.steps as f32;
        }

        (is_done, is_truncated, reward)
    }

    fn calculate_visited_paths(&self) -> HashMap<Direction, usize> {
        self.available_paths()
            .iter()
            .map(|(d, steps)| {
                (
                    *d,
                    *self
                        .visited
                        .get(
                            &self
                                .maze
                                .move_from(&*d, &self.current_location, *steps)
                                .unwrap(),
                        )
                        .unwrap_or(&0),
                )
            })
            .collect()
    }
}

#[pymethods]
impl Environment {
    pub fn take_action(&mut self, action: Action) -> (Vec<f32>, f32, bool, bool) {
        let old_location = self.current_location;
        let dir = Direction::from(action.direction);
        let old_direction = self.previous_direction;
        let steps_taken = self.move_from_current(&dir, action.run);
        let (is_done, is_truncated, reward) =
            self.calculate_reward_for_solving(old_location, old_direction);

        ActionResult {
            observation: Observation::new(&self, old_location),
            reward: (reward * (steps_taken as f32).max(1.0))
                * (action.run as f32 / self.config.python_config.mini_runs_per_episode as f32),
            is_done,
            is_truncated,
        }
        .flatten_and_scale(&self)
    }

    pub fn reset(&mut self) -> Vec<f32> {
        self.visited.clear();
        self.current_location = self.maze.start;
        Observation::new(&self, self.maze.get_starting_point()).flatten_and_scale_observation(&self)
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
