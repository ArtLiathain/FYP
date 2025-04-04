use std::{fs::File, io::{Error, Read}};

use maze_library::{
    environment::environment::Environment,
    maze_gen::maze_gen::{random_kruzkals_maze, random_wilson_maze},
};
use rand::{rng, seq::IteratorRandom};
use strum::IntoEnumIterator;

use crate::{
    solving_algorithms::{dfs_search::solve_maze_dfs, dijkstra::dijkstra_solve},
    MazeType, SolveAlgorithm,
};

pub fn read_environment_from_file(filename: &str) -> Result<Environment, Error> {
    let mut contents = String::new();

    let _ = match File::open(filename) {
        Ok(mut file_safe) => match file_safe.read_to_string(&mut contents) {
            Ok(_) => Ok(contents.clone()),
            Err(e) => {
                eprintln!("Error reading file: {}", e);
               Err(e)
            }
        },
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e)
        }
    };
    Ok(Environment::from_json(&contents).unwrap())
}

pub fn generate_environment_list(
    algorithm: &MazeType,
    width: usize,
    height: usize,
    count: usize,
) -> Vec<Environment> {
    let mut environments = vec![];
    for _ in 0..count {
        environments.push(generate_environment(algorithm, width, height));
    }
    environments
}

pub fn generate_environment(algorithm: &MazeType, width: usize, height: usize) -> Environment {
    let walls;
    let mut env = Environment::new(width, height);
    match algorithm {
        MazeType::Wilsons => walls = random_wilson_maze(&env.maze),
        MazeType::Kruzkals => walls = random_kruzkals_maze(&env.maze),
        MazeType::Random => {
            let mut rng = rng();
            let new_algorithm = &MazeType::iter()
                .filter(|algo| algo != &MazeType::Random) // Exclude the chosen variant
                .choose(&mut rng)
                .unwrap();

            return generate_environment(new_algorithm, width, height);
        }
    }
    env.maze.break_walls_for_path(walls);
    env
}

pub fn solve_maze(environment: &mut Environment, algorithm: &SolveAlgorithm) {
    match algorithm {
        SolveAlgorithm::Dfs => solve_maze_dfs(environment),
        SolveAlgorithm::Dijkstra => {
            dijkstra_solve(environment);
        }
    };
}
