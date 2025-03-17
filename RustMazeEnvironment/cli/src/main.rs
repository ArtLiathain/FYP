use std::collections::HashSet;

use cli::{Cli, Commands};
use handler_functions::{generate_environment_list, read_environment_from_file, solve_mazes};
use strum_macros::EnumIter;
mod cli;
mod handler_functions;
pub mod solving_algorithms;
use clap::{Parser, ValueEnum};
use log::info;
use macroquad::window::{next_frame, Conf};
use maze_library::{
    constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH}, direction::Direction, environment::environment::Environment, maze_gen::maze_gen::random_wilson_maze, render::render::{draw_maze, render_mazes}
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Renderer".to_owned(),
        window_width: WINDOW_WIDTH,   
        window_height: WINDOW_HEIGHT, 
        fullscreen: false,
        ..Default::default()
    }
}
#[derive(ValueEnum, Clone, Debug, Hash, Eq, PartialEq, EnumIter)]
pub enum MazeType {
    Kruzkals,
    Wilsons,
    Random,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SolveAlgorithm {
    Dfs,
    Dijkstra,
}

fn main() {
    env_logger::init(); 
    let cli = Cli::parse();
    let cell_size = 15.0;

    match cli.command {
        Commands::Solve {
            gen_algotithm,
            solve_algoithm,
            count,
            width,
            length,
        } => {
            info!("Starting maze solving process...");
            info!(
                "Generation Algorithm: {:?}, Solve Algorithm: {:?}",
                gen_algotithm, solve_algoithm
            );
            info!(
                "Maze count: {}, Width: {}, Height: {}",
                count, width, length
            );
            let mut temp = Environment::new(width, length);
            let walls = random_wilson_maze(&temp.maze);
            temp.maze.break_walls_for_path(walls);
            temp.weighted_graph = temp.maze.convert_to_weighted_graph();
            println!("{}", temp.steps);
            println!("{:?}", temp.current_location);
            println!("{:?}", temp.weighted_graph);
            temp.move_from_current(&Direction::random());
            temp.move_from_current(&Direction::random());
            temp.move_from_current(&Direction::random());
            temp.move_from_current(&Direction::random());
            temp.move_from_current(&Direction::random());
            temp.move_from_current(&Direction::random());
            println!("{}", temp.steps);
            println!("{:?}", temp.current_location);
            println!("{:?}", temp.available_paths());
            
            let mut environments = generate_environment_list(&gen_algotithm, width, length, count);
            solve_mazes(&mut environments, solve_algoithm);
            macroquad::Window::from_config(window_conf(), async move {
                // Game loop
                render_mazes(environments, cell_size).await;
            });
        }
        Commands::Display {
            files_location,
            start,
            count,
        } => {
            info!("Displaying maze from file...");
            info!("File Location: {}", files_location);
            info!("File Count: {}", count);
            let mut environments: Vec<Environment> = vec![];
            for i in start..start+count {
                let filename = format!("{}/solve{}.json",files_location, i);
                let environment = read_environment_from_file(&filename);
                environments.push(environment);
            }
            macroquad::Window::from_config(window_conf(), async move {
                // Game loop
                render_mazes(environments, cell_size).await;
            });
        }
        Commands::Test {  } => {
            let filename = format!("../mazeLogs/error_0.json");
            let environment = read_environment_from_file(&filename);
            macroquad::Window::from_config(window_conf(), async move {
                loop {
                    draw_maze(&environment,cell_size,&HashSet::new(),0, 0.0, 0.0).await;
                    next_frame().await;
                }
            });
        }
    }
}
