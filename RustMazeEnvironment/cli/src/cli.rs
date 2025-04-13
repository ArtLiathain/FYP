use clap::{Parser, Subcommand};

use crate::{ExploreAlgorithm, MazeType, SolveAlgorithm};

#[derive(Parser)]
#[command(name = "RustMazeCLI")]
#[command(about = "A CLI tool for solving and displaying mazes", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Solve a maze with given parameters
    Solve {
        /// Maze exploring algorithm
        #[arg(short, long, value_enum, default_value_t=ExploreAlgorithm::Random)]
        explore_algoithm: ExploreAlgorithm,

        /// maze generation algotithm
        #[arg(short, long, value_enum, default_value_t=MazeType::Random)]
        gen_algotithm: MazeType,

        /// Maze solving algorithm
        #[arg(short, long, value_enum, default_value_t=SolveAlgorithm::Dijkstra)]
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

        /// additional walls to remove
        #[arg(short, long, default_value_t = 0)]
        removed_walls: usize,
    },

    /// Display a maze with given parameters
    Display {
        /// Input string (e.g., display
        #[arg(short, long, required = true)]
        files_location: String,

        /// Starting number
        #[arg(short, long, default_value_t = 0)]
        start: usize,

        /// Number of files to iterate over
        #[arg(short, long, default_value_t = 5)]
        count: usize,
    },
    Compare {
        #[arg(short, long, value_enum, default_values_t=vec![SolveAlgorithm::Dfs, SolveAlgorithm::Dijkstra], num_args = 0..)]
        solve_algoithms: Vec<SolveAlgorithm>,

        #[arg(short, long, value_enum, default_value_t=MazeType::Random)]
        gen_algotithm: MazeType,

        #[arg(short, long, required = false)]
        files_location: Option<String>,

        /// number of mazes to solve
        #[arg(short, long, default_value_t = 10)]
        count: usize,

        #[arg(short, long, default_value_t = 20)]
        width: usize,
        /// Heigh of maze
        #[arg(short, long, default_value_t = 20)]
        length: usize,
    },
    Test {},
}
