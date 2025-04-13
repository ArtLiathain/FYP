pub mod environment {
    use crate::{
        direction::{direction_between, Direction},
        environment_config::EnvConfig,
        maze::maze::Maze,
    };
    use log::error;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    pub type Coordinate = (usize, usize);

    #[cfg_attr(feature = "python", pyo3::pyclass)]
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Environment {
        pub path_followed: Vec<(Coordinate, usize)>,
        pub current_location: Coordinate,
        pub previous_direction: Option<Direction>,
        pub maze: Maze,
        pub steps: usize,
        pub config: EnvConfig,
        #[serde(skip)]
        pub visited: HashMap<Coordinate, usize>,
        #[serde(skip)]
        pub weighted_graph: HashMap<Coordinate, HashMap<Direction, usize>>,
    }

    pub struct ReportCard {
        pub total_steps: usize,
        pub best_run_penalty: usize,
        pub best_run_steps: usize,
        pub best_run: usize,
    }

    impl Environment {
        pub fn new(env_config: EnvConfig) -> Environment {
            let maze = Maze::init_maze(env_config.maze_width, env_config.maze_height);
            Environment {
                current_location: maze.start,
                path_followed: vec![(maze.get_starting_point(), 0)],
                previous_direction: None,
                config: env_config,
                visited: HashMap::from([(maze.get_starting_point(), 0)]),
                maze,
                steps: 0,
                weighted_graph: HashMap::new(),
            }
        }
    }

    impl Environment {
        pub fn move_from_current(&mut self, direction: &Direction, run: usize) {
            let steps = self
                .weighted_graph
                .get(&self.current_location)
                .and_then(|inner_map| inner_map.get(direction))
                .copied() // Extracts the value as usize instead of &usize
                .unwrap_or(0);
            self.steps += steps;
            for i in 0..steps {
                let intermediary_step = self
                    .maze
                    .move_from(direction, &self.current_location, i)
                    .unwrap();
                self.path_followed.push((intermediary_step, run));
                *self.visited.entry(intermediary_step).or_insert(0) += 1;
            }
            match self
                .maze
                .move_from(direction, &self.current_location, steps)
            {
                Ok(new_loc) => {
                    self.path_followed.push((new_loc, run));
                    *self.visited.entry(new_loc).or_insert(0) += 1;
                    self.previous_direction = Some(*direction);
                    self.current_location = new_loc;
                }
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

        pub fn get_current_run(&self) -> usize {
            if self.path_followed.is_empty() {
                return 0;
            }
            self.path_followed[self.path_followed.len() - 1].1
        }

        pub fn generate_report_card(&self) -> ReportCard {
            let mut best_score_run = (usize::MAX, usize::MAX, 0);
            for i in 0..self.get_current_run() {
                let (total_run_steps, total_run_penalty) = self.calculate_run_score(i);
                if total_run_steps + total_run_penalty < best_score_run.0 + best_score_run.1 {
                    best_score_run = (total_run_steps, total_run_penalty, i)
                }
            }
            ReportCard {
                total_steps: self.steps,
                best_run_steps: best_score_run.0,
                best_run_penalty: best_score_run.1,
                best_run: best_score_run.2,
            }
        }

        pub fn calculate_run_score(&self, run_to_score: usize) -> (usize, usize) {
            let filtered_path: Vec<Coordinate> = self
                .path_followed
                .iter()
                .filter(|(_, run)| *run == run_to_score)
                .map(|(coord, _)| *coord)
                .collect();
            let mut prev_direction = direction_between(filtered_path[0], filtered_path[1]).unwrap();
            let mut total_run_steps = 0;
            let mut total_run_penalty = 0;
            for index in 2..filtered_path.len() {
                let direction =
                    direction_between(filtered_path[index - 1], filtered_path[index]).unwrap();
                total_run_penalty += direction.turn_amount(&prev_direction);
                total_run_steps += self
                    .weighted_graph
                    .get(&filtered_path[index - 1])
                    .unwrap()
                    .get(&direction)
                    .unwrap();
                prev_direction = direction;
            }

            (total_run_steps, total_run_penalty)
        }
    }
}
