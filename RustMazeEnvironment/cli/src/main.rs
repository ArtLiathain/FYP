
use clap::Parser;
use cli::{Cli, Commands};
use handler_functions::{
    extract_prefix, generate_environment_list, read_environment_from_file,
};
use log::info;
use macroquad::window::Conf;
use maze_library::{
    constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH}, environment::environment::Environment, exploring_algorithms::explore_handler::explore_maze_with, render_system::render::render::render_mazes, solving_algorithms::{dijkstra::dijkstra_graph, solve_handler::select_maze_solve_algorithm}
};
mod cli;
mod handler_functions;

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Renderer".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
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
            removed_walls,
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
            let mut environments = generate_environment_list(
                &gen_algotithm,
                width,
                length,
                count,
                removed_walls,
                Some(22),
            );

            for mut environment in environments.iter_mut() {
                environment.weighted_graph = environment.maze.convert_to_weighted_graph(None, true);
                explore_maze_with(&mut environment, &explore_algoithm);
                let path = select_maze_solve_algorithm(environment, &solve_algoithm);
                environment.move_path_vec(&path, environment.get_current_run() + 1);
            }
            macroquad::Window::from_config(window_conf(), async move {
                // Game loop
                render_mazes(environments, cell_size, false).await;
            });
        }
        Commands::Display {
            count,
            filename,
        } => {
            info!("Displaying maze from file...");
            info!("File Location: {}", filename);
            info!("File Count: {}", count);
            let mut environments: Vec<Environment> = vec![];
            let (prefix, start) = extract_prefix(&filename);
            for i in start..start + count {
                let filename = format!("{}{}.json", prefix, i);
                let environment = read_environment_from_file(&filename).expect(&format!("File {} does not exist", filename));
                environments.push(environment);
            }
            macroquad::Window::from_config(window_conf(), async move {
                // Game loop
                render_mazes(environments, cell_size, false).await;
            });
        }
        Commands::ColouredDisplay {
            count,
            filename,
        } => {
            info!("Displaying maze from file...");
            info!("File Location: {}", filename);
            info!("File Count: {}", count);
            let mut environments: Vec<Environment> = vec![];
            let (prefix, start) = extract_prefix(&filename);
            for i in start..start + count {
                let filename = format!("{}{}.json", prefix, i);
                let environment = read_environment_from_file(&filename).unwrap();
                environments.push(environment);
            }
            macroquad::Window::from_config(window_conf(), async move {
                // Game loop
                render_mazes(environments, cell_size, true).await;
            });
        }
        Commands::ShowGenBias {
            gen_algotithm,
            count,
            width,
            length,
            removed_walls,
        } => {
            info!("Generating mazes...");
            let mut environments = generate_environment_list(
                &gen_algotithm,
                width,
                length,
                count,
                removed_walls,
                None,
            );
            environments.iter_mut().for_each(|env| {
                env.weighted_graph = env.maze.convert_to_weighted_graph(None, false);
                let path_graph = dijkstra_graph(&env, *env.maze.end.iter().next().unwrap())
                    .into_iter()
                    .map(|(k, v)| (k, v.0)) // Take the first element (usize) from the tuple
                    .collect();
                env.overall_visited = path_graph;
            });

            macroquad::Window::from_config(window_conf(), async move {
                // Game loop
                render_mazes(environments, cell_size, true).await;
            });
        }
    }
}
