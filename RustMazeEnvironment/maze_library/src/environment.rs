pub mod environment {
    use crate::{
        direction::{direction_between, Direction},
        environment_config::EnvConfig,
        map_vec_conversion::map_vec_conversion,
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
        #[serde(with = "map_vec_conversion")]
        pub overall_visited: HashMap<Coordinate, usize>,
        #[serde(skip)]
        pub weighted_graph: HashMap<Coordinate, HashMap<Direction, usize>>,
    }

    pub fn calcualte_score_for_coordinate_vector(
        path: &Vec<Coordinate>,
        weighted_graph: &HashMap<Coordinate, HashMap<Direction, usize>>,
    ) -> (usize, usize, usize, f32) {
        let mut direction_map = HashMap::new();
        let mut reverse_count = 0;
        let mut hit_count = 0;
        let mut turn_count = 0;
        let mut prev_direction = direction_between(path[0], path[1]).unwrap_or(Direction::North);
        let mut total_run_steps = 0;
        let mut total_run_penalty = 0;
        for index in 2..path.len() {
            let direction = direction_between(path[index - 1], path[index]);

            if direction.is_none() {
                hit_count += 1;
                continue;
            }
            *direction_map.entry(direction.unwrap()).or_insert(0) += 1;

            let count_steps = weighted_graph.get(&path[index - 1]);
            if count_steps.is_none() {
                hit_count += 1;
                continue;
            }
            let turn_penalty = direction
                .expect("Direction overall")
                .turn_amount(&prev_direction);
            if turn_penalty > 0 {
                turn_count += 1;
            } else {
                *direction_map.entry(prev_direction).or_insert(0) += 1;
            }
            if turn_penalty == 2 {
                reverse_count += 1;
            }
            total_run_penalty += turn_penalty;

            total_run_steps += count_steps
                .expect("Hashmap get")
                .get(&direction.unwrap())
                .expect("direction in nested hashmap");
            prev_direction = direction.unwrap();
        }
        (
            total_run_steps + total_run_penalty,
            hit_count,
            reverse_count,
            total_run_steps as f32 / turn_count as f32,
        )
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
                overall_visited: HashMap::from([(maze.get_starting_point(), 0)]),
                maze,
                steps: 0,
                weighted_graph: HashMap::new(),
            }
        }
    }

    impl Environment {
        pub fn move_from_current(&mut self, direction: &Direction, run: usize) -> usize {
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
                *self.overall_visited.entry(intermediary_step).or_insert(0) += 1;
            }
            match self
                .maze
                .move_from(direction, &self.current_location, steps)
            {
                Ok(new_loc) => {
                    self.path_followed.push((new_loc, run));
                    *self.visited.entry(new_loc).or_insert(0) += 1;
                    *self.overall_visited.entry(new_loc).or_insert(0) += 1;
                    self.previous_direction = Some(*direction);
                    self.current_location = new_loc;
                    return steps;
                }
                Err(_e) => {
                    error!("MOVE ERROR WITH WEIGHTED GRAPH");
                }
            }
            0
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

        pub fn calculate_run_score(&self, run_to_score: usize) -> (usize, usize, usize, f32, bool) {
            let filtered_path: Vec<Coordinate> = self
                .path_followed
                .iter()
                .filter(|(_, run)| *run == run_to_score)
                .map(|(coord, _)| *coord)
                .collect();
            let (total_run_score, hit_count, reverse_count, average_path_length) =
                calcualte_score_for_coordinate_vector(&filtered_path, &self.weighted_graph);
            (
                total_run_score,
                hit_count,
                reverse_count,
                average_path_length as f32,
                self.maze
                    .end
                    .contains(&filtered_path[filtered_path.len() - 1]),
            )
        }

        pub fn move_path_vec(&mut self, path: &Vec<(Coordinate, Direction)>, run: usize) {
            for (_, direction) in path {
                self.move_from_current(direction, run);
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use crate::environment_config::PythonConfig;

        use super::*;

        #[test]
        fn test_json() {
            let env = Environment::new(EnvConfig {
                maze_width: 10,
                maze_height: 10,
                python_config: PythonConfig::default(),
            });
            let json = env.to_json();
            let parsed = Environment::from_json(&json);
            assert!(parsed.is_ok());
        }
    }
}
