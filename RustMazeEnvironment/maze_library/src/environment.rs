pub mod environment {
    use crate::{
        direction::{direction_between, Direction},
        environment_config::EnvConfig,
        map_vec_conversion::map_vec_conversion,
        maze::maze::Maze, maze_gen::maze_gen_handler::MazeType,
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
        pub total_steps: usize,
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
    ) -> (usize, usize, usize) {
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

            total_run_steps += match count_steps.expect("Hashmap get").get(&direction.unwrap()) {
                Some(steps) => steps,
                None => {
                    panic!("ERROR");
                }
            };

            prev_direction = direction.unwrap();
        }
        (
            total_run_steps + total_run_penalty,
            hit_count,
            reverse_count,
        )
    }

    impl Environment {
        pub fn new(env_config: EnvConfig) -> Environment {
            let maze = Maze::init_maze(env_config.maze_width, env_config.maze_height);
            Environment {
                current_location: maze.start,
                previous_direction: None,
                config: env_config,
                path_followed: Vec::from([(maze.get_starting_point(), 0)]),
                visited: HashMap::from([(maze.get_starting_point(), 1)]),
                overall_visited: HashMap::from([(maze.get_starting_point(), 1)]),
                maze,
                steps: 0,
                total_steps: 0,
                weighted_graph: HashMap::new(),
            }
        }
    }

    impl Environment {
        pub fn mark_nearby_as_visited(&mut self) {
            for direction in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let new_coords = match self.maze.move_from_with_walls(&direction, &self.current_location, 1) {
                    Ok(new) => new,
                    Err(_) => continue,
                };
                if self.overall_visited.contains_key(&new_coords) {
                    continue;
                }
                self.overall_visited.insert(new_coords, 0);
            }
        }

        pub fn move_from_current(&mut self, direction: &Direction, run: usize) -> usize {
            let steps = *self
                .weighted_graph
                .get(&self.current_location)
                .and_then(|inner_map| inner_map.get(direction))
                .unwrap_or(&0);

            if steps == 0 {
                self.steps += 1;
                self.path_followed.push((self.current_location, run));
                *self.visited.entry(self.current_location).or_insert(0) += 1;
                *self.overall_visited.entry(self.current_location).or_insert(0) += 1;
                return 0;
            }
            let current_path_length = self.path_followed.len();
            for i in 0..steps {
                let intermediary_step =
                    match self
                        .maze
                        .move_from_with_walls(direction, &self.current_location, 1)
                    {
                        Ok(new_loc) => new_loc,
                        Err(e) => {
                            error!("MOVE ERROR WITH WEIGHTED GRAPH: {:?}", e);
                            println!("ERROR {:?}", e);
                            return i; // Or use a custom error type
                        }
                    };
                self.steps += 1;
                self.path_followed.push((intermediary_step, run));
                *self.visited.entry(intermediary_step).or_insert(0) += 1;
                *self.overall_visited.entry(intermediary_step).or_insert(0) += 1;
                self.current_location = intermediary_step;
            }
            self.mark_nearby_as_visited();
            self.previous_direction = Some(*direction);
            if current_path_length >= self.path_followed.len() {
                println!("HUGE ERROR IN THIS STUPID FUNCTION");
            }
            steps
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

        pub fn calculate_run_score(&self, run_to_score: usize) -> (usize, usize, usize, bool) {
            let filtered_path: Vec<Coordinate> = self
                .path_followed
                .iter()
                .filter(|(_, run)| *run == run_to_score)
                .map(|(coord, _)| *coord)
                .collect();
            if filtered_path.len() <= 1 {
                return (0, 0, 0, false);
            }
            let (total_run_score, hit_count, reverse_count) =
                calcualte_score_for_coordinate_vector(&filtered_path, &self.weighted_graph);
            (
                total_run_score,
                hit_count,
                reverse_count,
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

        #[test]
        fn test_movement() {
            let mut env = Environment::new(EnvConfig {
                maze_width: 10,
                maze_height: 10,
                python_config: PythonConfig::default(),
            });

            // Allow eastward movement for 2 steps
            env.maze.grid[0][9].walls.remove(&Direction::East);
            env.maze.grid[1][9].walls.remove(&Direction::West);
            env.maze.grid[1][9].walls.remove(&Direction::East);
            env.maze.grid[2][9].walls.remove(&Direction::West);

            // Update agent's weighted graph to allow 2 eastward steps
            let mut east_map = HashMap::new();
            east_map.insert(Direction::East, 2);
            env.weighted_graph.insert((0, 9), east_map);
            env.current_location = (0, 9);

            let steps_taken = env.move_from_current(&Direction::East, 0);

            assert_eq!(steps_taken, 2);
            assert_eq!(env.current_location, (2, 9));
            assert_eq!(env.path_followed.len(), 3);
        }
    }
}
