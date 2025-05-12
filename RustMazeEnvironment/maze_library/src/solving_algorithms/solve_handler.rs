use std::str::FromStr;

use clap::ValueEnum;

use crate::{direction::{direction_between, Direction}, environment::environment::{Coordinate, Environment}};

use super::{dfs_search::solve_maze_dfs, dijkstra::dijkstra_solve};

#[derive(ValueEnum, Clone, Debug)]
pub enum SolveAlgorithm {
    Dfs,
    Dijkstra,
}

impl FromStr for SolveAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dfs" => Ok(SolveAlgorithm::Dfs),
            "dijkstra" => Ok(SolveAlgorithm::Dijkstra),
            _ => Err(()),
        }
    }
}

pub fn select_maze_solve_algorithm(environment: &Environment, algorithm: &SolveAlgorithm) -> Vec<(Coordinate, Direction)>{
    let maze = &environment.maze;
    let path = match algorithm {
        SolveAlgorithm::Dfs => {
            solve_maze_dfs(environment, *environment.maze.end.iter().next().unwrap())
        }
        SolveAlgorithm::Dijkstra => dijkstra_solve(
            &environment,
            maze.start,
            *environment.maze.end.iter().next().unwrap(),
        ),
    };
    let mut vec_with_direction = vec![];
    for index in 1..path.len() {
        let direction = direction_between(path[index - 1], path[index]).unwrap();
        vec_with_direction.push((path[index - 1], direction));
    }
    vec_with_direction
}
