use rand::{rng, seq::IteratorRandom};
use solving_algorithms::dfs_search::maze_solve::solve_maze_dfs;
use std::{
    fs::File,
    io::Read,
};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;
mod cli;
pub mod solving_algorithms;
use clap::{Parser, Subcommand, ValueEnum};
use log::{info, warn};
use macroquad::window::Conf;
use maze_library::{
    constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    environment::environment::Environment,
    maze_gen::maze_gen::{random_kruzkals_maze, random_wilson_maze},
    render::render::render_mazes,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Renderer".to_owned(),
        window_width: WINDOW_WIDTH,   // Set the desired width
        window_height: WINDOW_HEIGHT, // Set the desired height
        fullscreen: false,
        ..Default::default()
    }
}
#[derive(ValueEnum, Clone, Debug, Hash, Eq, PartialEq, EnumIter)]
enum MazeType {
    Kruzkals,
    Wilsons,
    Random,
}

#[derive(ValueEnum, Clone, Debug)]
enum SolveAlgorithm {
    Dfs,
    Bfs,
}

#[derive(Parser)]
#[command(name = "RustMazeCLI")]
#[command(about = "A CLI tool for solving and displaying mazes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve a maze with given parameters
    Solve {
        /// maze generation algotithm
        #[arg(short, long, value_enum, default_value_t=MazeType::Random)]
        gen_algotithm: MazeType,

        /// Maze solving algorithm
        #[arg(short, long, value_enum, default_value_t=SolveAlgorithm::Dfs)]
        solve_algoithm: SolveAlgorithm,

        /// number of mazes to solve
        #[arg(short, long, default_value_t = 10)]
        count: usize,
        /// Width of maze
        #[arg(short, long, default_value_t = 20)]
        width: usize,
        /// Heigh of maze
        #[arg(short, long, default_value_t = 20)]
        length: usize,
    },

    /// Display a maze with given parameters
    Display {
        /// Input string (e.g., display
        #[arg(short, long, required = true)]
        file_location: String,

        /// Number of files to iterate over
        #[arg(short, long, default_value_t= 5)]
        file_count: usize,
    },
}


fn main() {
    env_logger::init(); // Initialize logging system
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
            let mut environments = generate_environment_list(&gen_algotithm, width, length, count);
            solve_mazes(&mut environments, solve_algoithm);
            macroquad::Window::from_config(
                window_conf(),
                async move {
                    // Game loop
                    render_mazes(environments, cell_size).await;
                }
            );
        }
        Commands::Display {
            file_location,
            file_count,
        } => {
            info!("Displaying maze from file...");
            info!("File Location: {}", file_location);
            info!("File Count: {}", file_count);
            let mut environments: Vec<Environment> = vec![];
            for i in 0..30 {
                let filename = format!("../mazeLogs/solve{}.json", i);
                let environment = read_environment_from_file(&filename);
                environments.push(environment);
            }
            macroquad::Window::from_config(
                window_conf(),
                async move {
                    // Game loop
                    render_mazes(environments, cell_size).await;
                }
            );
        }
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

fn generate_environment_list(
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

fn generate_environment(algorithm: &MazeType, width: usize, height: usize) -> Environment {
    let mut walls = vec![];
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

fn solve_mazes(environments: &mut Vec<Environment>, algorithm: SolveAlgorithm) {

    let _ = environments.iter_mut().for_each(|env| {
        match algorithm {
            SolveAlgorithm::Dfs => solve_maze_dfs(env),
            SolveAlgorithm::Bfs => {warn!("PANICKING")}
        };
    });
}
