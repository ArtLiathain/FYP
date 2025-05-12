use std::collections::HashMap;

use pyo3::pyclass;

use crate::{
    direction::Direction,
    environment::environment::{Coordinate, Environment},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Observation {
    #[pyo3(get)]
    available_paths: HashMap<Direction, usize>,
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
    end_node: (f32, f32),
    #[pyo3(get)]
    is_exploring: bool,
}
pub fn calculate_manhattan_distance(pos1: Coordinate, pos2: (f32, f32)) -> f32 {
    (pos1.0 as f32 - pos2.0).abs() + (pos1.1 as f32 - pos2.1).abs()
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
            available_paths: env.available_paths(),
            current_location: env.current_location,
            end_node: env.maze.get_perfect_end_centre(),
            previous_location,
            is_exploring: env.config.python_config.mini_explore_runs_per_episode > 0,
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
            let mut direction_encoding = vec![0.0; 4]; // quasi-one-hot

            let steps = *self.available_paths.get(dir).unwrap_or(&0) as f32;

            let norm = match dir {
                Direction::North | Direction::South => env.maze.height as f32,
                Direction::East | Direction::West => env.maze.width as f32,
            };

            let index = match dir {
                Direction::North => 0,
                Direction::East => 1,
                Direction::South => 2,
                Direction::West => 3,
            };

            direction_encoding[index] = steps / norm;

            vec.extend(direction_encoding);
        }
        let visited_paths = self.calculate_visited_paths(env);
        for dir in direction_vec.iter() {
            if *visited_paths.get(dir).unwrap_or(&0) > 0 {
                vec.push(0.0);
            }
            else {
                vec.push(1.0);
            }
        }

        vec.push(self.current_location.0 as f32 / (env.maze.width as f32 - 1.0));
        vec.push(self.current_location.1 as f32 / (env.maze.height as f32 - 1.0));
        vec.push(self.end_node.0 / (env.maze.width as f32 - 1.0));
        vec.push(self.end_node.1 / (env.maze.height as f32 - 1.0));
        vec.push(self.previous_direction as f32);
        vec.push(self.previous_location.0 as f32 / (env.maze.width as f32 - 1.0));
        vec.push(self.previous_location.1 as f32 / (env.maze.height as f32 - 1.0));
        vec.push(self.manhattan_distance / (env.maze.width + env.maze.height) as f32);
        vec.push((self.goal_dxdy.0 / (env.maze.width as f32 / 2.0) + 1.0) / 2.0);
        vec.push((self.goal_dxdy.1 / (env.maze.height as f32 / 2.0) + 1.0) / 2.0);

        vec.push(
            if env.get_current_run() >= env.config.python_config.mini_explore_runs_per_episode {
                1.0
            } else {
                0.0
            },
        );
        vec.extend(self.get_5x5_features(&env));
        vec
    }

    fn get_5x5_features(&self, env: &Environment) -> Vec<f32> {
        let mut features = Vec::with_capacity(5 * 5 * 7);

        for dy in -2..=2 {
            for dx in -2..=2 {
                let x = env.current_location.0 as i32 + dx;
                let y = env.current_location.1 as i32 + dy;

                match env.maze.in_bounds((x, y)) {
                    true => {
                        let coord = (x as usize, y as usize);
                        let cell = env.maze.get_cell(coord); // -> [bool; 4]

                        features.extend([
                            cell.walls.contains(&Direction::North) as u8 as f32,
                            cell.walls.contains(&Direction::South) as u8 as f32,
                            cell.walls.contains(&Direction::East) as u8 as f32,
                            cell.walls.contains(&Direction::West) as u8 as f32,
                        ]);
                        features.push(if env.overall_visited.contains_key(&coord) {
                            1.0
                        } else {
                            0.0
                        });
                        features.push(0.0); // not out of bounds
                        features.push(if coord == env.current_location {
                            1.0
                        } else {
                            0.0
                        });
                    }
                    false =>
                    // Out of bounds: all walls, not visited, out_of_bounds = 1
                    {
                        features.extend([1.0, 1.0, 1.0, 1.0]); // walls
                        features.push(0.0); // visited
                        features.push(1.0); // out of bounds
                        features.push(0.0);
                    } // agent,
                }
            }
        }

        features
    }

    fn calculate_visited_paths(&self, env: &Environment) -> HashMap<Direction, usize> {
        env.available_paths()
            .iter()
            .map(|(d, steps)| {
                (
                    *d,
                    *env.visited
                        .get(
                            &env.maze
                                .move_from(&*d, &env.current_location, *steps)
                                .unwrap(),
                        )
                        .unwrap_or(&0),
                )
            })
            .collect()
    }
}
