use std::{
    fs::File,
    io::{Error, Read},
};

use maze_library::{
    environment::environment::Environment, environment_config::EnvConfig, exploring_algorithms::{explore_handler::ExploreAlgorithm, wall_following::follow_wall_explore}, maze_gen::maze_gen_handler::{select_maze_algorithm, MazeType}, solving_algorithms::{dfs_search::solve_maze_dfs, dijkstra::dijkstra_solve, solve_handler::{select_maze_solve_algorithm, SolveAlgorithm}}
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
            return Err(e);
        }
    };
    Ok(Environment::from_json(&contents).unwrap())
}

pub fn generate_environment_list(
    algorithm: &MazeType,
    width: usize,
    height: usize,
    count: usize,
    removed_walls: usize,
    rng_seed: Option<u64>,
) -> Vec<Environment> {
    let mut environments = vec![];
    for _ in 0..count {
        environments.push(generate_environment(
            algorithm,
            width,
            height,
            removed_walls,
            rng_seed,
        ));
    }
    environments
}

pub fn generate_environment(
    algorithm: &MazeType,
    width: usize,
    height: usize,
    removed_walls: usize,
    rng_seed: Option<u64>,
) -> Environment {
    let mut env = Environment::new(EnvConfig::new_rust_config(width, height));
    let walls = select_maze_algorithm(&env.maze, rng_seed, algorithm);

    env.maze.break_walls_for_path(walls);
    let extra_walls = env.maze.break_random_walls(removed_walls);
    env.maze.break_walls_for_path(extra_walls);
    env
}
 
pub fn solve_maze(env : &mut Environment, algorithm: &SolveAlgorithm) {
    let path = select_maze_solve_algorithm(env, algorithm);
    
}   

