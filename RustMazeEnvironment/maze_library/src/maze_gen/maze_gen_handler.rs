use clap::ValueEnum;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::str::FromStr;
use strum_macros::EnumIter;

use crate::{direction::Direction, environment::environment::Coordinate, maze::maze::Maze};

use super::{
    binary_tree::random_binary_maze, growing_tree::growing_tree_maze,
    kruzkals::random_kruzkals_maze, wilsons::random_wilson_maze,
};

#[derive(ValueEnum, Clone, Debug, Hash, Eq, PartialEq, EnumIter)]
pub enum MazeType {
    Kruzkals,
    Wilsons,
    RecursiveBacktracker,
    Prims,
    BinaryTree,
}

impl FromStr for MazeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kruzkals" => Ok(MazeType::Kruzkals),
            "wilsons" => Ok(MazeType::Wilsons),
            "recursive-backtracker" => Ok(MazeType::RecursiveBacktracker),
            "prims" => Ok(MazeType::Prims),
            "binary-tree" => Ok(MazeType::BinaryTree),
            _ => Err(()),
        }
    }
}

pub fn select_maze_algorithm(
    maze: &Maze,
    rng_seed: Option<u64>,
    algorithm: &MazeType,
) -> Vec<(Coordinate, Direction)> {
    let rng = match rng_seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_rng(&mut rand::rng()),
    };
    match algorithm {
        MazeType::Wilsons => random_wilson_maze(maze, rng),
        MazeType::Kruzkals => random_kruzkals_maze(maze, rng),
        MazeType::RecursiveBacktracker => growing_tree_maze(maze, rng, &|list| list.last().unwrap()),
        MazeType::BinaryTree => random_binary_maze(maze, rng),
        MazeType::Prims => growing_tree_maze(maze, rng.clone(), &|list| {
            &list[rng.clone().random_range(0..list.len())]
        }),
    }
}
