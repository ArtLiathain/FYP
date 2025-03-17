use clap::{Parser, Subcommand};

use crate::{MazeType, SolveAlgorithm};

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
        files_location: String,

        /// Starting number
        #[arg(short, long, default_value_t = 0)]
        start: usize,

        /// Number of files to iterate over
        #[arg(short, long, default_value_t= 5)]
        count: usize,
    },
    Test{
        
    }
}