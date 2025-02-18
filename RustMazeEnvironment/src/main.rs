use std::{
    collections::HashSet,
    env,
    fs::File,
    io::Read,
    thread::{self, sleep},
    time::Duration,
};

pub use constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use environment::environment::Environment;
use macroquad::window::Conf;
use maze::maze::{Direction, Maze};
use maze_gen::maze_gen::{random_kruzkals_maze, random_wilson_maze};
use render::render::{render_maze, render_mazes};
pub mod maze_gen;
pub mod maze_solve;
pub mod maze;
pub mod render;
pub mod environment;
pub mod constants;

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Renderer".to_owned(),
        window_width: WINDOW_WIDTH,  // Set the desired width
        window_height: WINDOW_HEIGHT, // Set the desired height
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let cell_size = 20.0;
    let mut environments: Vec<Environment> = vec![];
    for i in 0..30 {
        let filename = format!("../mazeLogs/solve{}.json", i);
        let environment = read_environment_from_file(&filename);
        environments.push(environment);
    }
    render_mazes(environments, cell_size).await;
   
}

async fn render_maze_loop(environment: &Environment, cell_size: f32) {
    let mut vistied = HashSet::new();
    for i in 0..environment.steps {
        render_maze(&environment, &vistied, cell_size, i).await;
        vistied.insert(environment.path_followed[i]);
        thread::sleep(Duration::from_millis(50));

    }
    
}

fn read_environment_from_file(filename: &str) -> Environment {
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
            Err(e)
        }
    };
    Environment::from_json(&contents).unwrap()
}

fn select_maze_gen_algorithm(algorithm: &str, maze: &mut Maze) -> Vec<((usize, usize), Direction)> {
    let algorithm_lower = algorithm.to_lowercase();
    if algorithm_lower == "wilson" {
        return random_wilson_maze(maze);
    } else if algorithm_lower == "kruzkal" {
        return random_kruzkals_maze(maze);
    }
    random_kruzkals_maze(maze)
}


