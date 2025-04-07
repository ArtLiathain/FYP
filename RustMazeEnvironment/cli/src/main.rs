use std::collections::HashSet;

use cli::{Cli, Commands};
use handler_functions::{
    explore_maze, generate_environment, generate_environment_list, read_environment_from_file,
    solve_maze,
};
use strum_macros::EnumIter;
mod cli;
pub mod exploring_algorithms;
mod handler_functions;
pub mod solving_algorithms;
use clap::{Parser, ValueEnum};
use log::info;
use macroquad::window::{next_frame, Conf};
use maze_library::{
    constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    environment::environment::Environment,
    maze_gen::maze_gen::break_random_walls,
    render::render::{draw_maze, render_mazes},
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
#[derive(ValueEnum, Clone, Debug, Hash, Eq, PartialEq, EnumIter)]
pub enum ExploreAlgorithm {
    WallFollowing,
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
            explore_algoithm,
            gen_algotithm,
            solve_algoithm,
            count,
            width,
            length,
        } => {
            info!("Starting maze solving process...");
            info!(
                "Generation Algorithm: {:?}, Solve Algorithm: {:?}, Exploration Algorithm: {:?}",
                gen_algotithm, solve_algoithm, explore_algoithm
            );
            info!(
                "Maze count: {}, Width: {}, Height: {}",
                count, width, length
            );
            let mut environments = generate_environment_list(&gen_algotithm, width, length, count);

            for mut environment in environments.iter_mut() {
                environment.weighted_graph = environment.maze.convert_to_weighted_graph();
                explore_maze(&mut environment, &explore_algoithm);
                println!("explored");
                solve_maze(&mut environment, &solve_algoithm);
                println!("solved");
            }
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
            for i in start..start + count {
                let filename = format!("{}/solve{}.json", files_location, i);
                let environment = read_environment_from_file(&filename).unwrap();
                environments.push(environment);
            }
            macroquad::Window::from_config(window_conf(), async move {
                // Game loop
                render_mazes(environments, cell_size).await;
            });
        }
        Commands::Compare {
            solve_algoithms,
            gen_algotithm,
            files_location,
            count,
            width,
            length,
        } => {
            let mut environment: Environment;

            match read_environment_from_file(&files_location.unwrap_or("".to_string())) {
                Ok(env) => environment = env,
                Err(_) => {
                    environment = generate_environment(&gen_algotithm, width, length);
                    let extra_walls = break_random_walls(&mut environment.maze, 40);
                    environment.maze.break_walls_for_path(extra_walls);
                }
            }
            let mut environments = vec![environment; count];
            for (index, mut environment) in environments.iter_mut().enumerate() {
                solve_maze(
                    &mut environment,
                    &solve_algoithms[index % solve_algoithms.len()],
                );
            }
            macroquad::Window::from_config(window_conf(), async move {
                // Game loop
                render_mazes(environments, cell_size).await;
            });
        }
        Commands::Test {} => {
            let filename = format!("../mazeLogs/error_0.json");
            let environment = read_environment_from_file(&filename).unwrap();
            macroquad::Window::from_config(window_conf(), async move {
                loop {
                    draw_maze(&environment, cell_size, &HashSet::new(), 0, 0.0, 0.0).await;
                    next_frame().await;
                }
            });
        }
    }
}
