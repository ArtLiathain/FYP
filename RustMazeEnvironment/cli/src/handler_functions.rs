use std::{
    fs::File,
    io::{Error, Read},
};

use maze_library::{
    environment::environment::Environment,
    environment_config::EnvConfig,
    maze_gen::maze_gen_handler::{select_maze_algorithm, MazeType},
};
use regex::Regex;

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

pub fn extract_prefix(path: &str) -> (String, usize) {
    // Define the regular expression to capture everything up until the last number and .json
    let re = Regex::new(r"^(.*\/[a-zA-Z0-9_]*?)(\d+)\.json$").unwrap();

    // Apply the regex to the input path
    if let Some(captures) = re.captures(path) {
        // The first captured group is the path and name up to the number
        // The second captured group is the number
        let path_and_name = captures.get(1).unwrap().as_str().to_string();
        let number = captures.get(2).unwrap().as_str().to_string();
        match number.parse::<usize>() {
            Ok(number) => (path_and_name, number),
            Err(_) => panic!("filename must follow <NAME><NUMBER>.json"), // Return None if the regex didn't match
        }
    } else {
        panic!("filename must follow <NAME><NUMBER>.json") // Return None if the regex didn't match
    }
}
