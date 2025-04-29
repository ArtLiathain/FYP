use std::str::FromStr;

use clap::ValueEnum;
use strum_macros::EnumIter;

use crate::environment::environment::Environment;

use super::wall_following::follow_wall_explore;

#[derive(ValueEnum, Clone, Debug, Hash, Eq, PartialEq, EnumIter)]
pub enum ExploreAlgorithm {
    WallFollowing,
    None,
}

impl FromStr for ExploreAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wall-following" => Ok(ExploreAlgorithm::WallFollowing),
            "none" => Ok(ExploreAlgorithm::None),
            _ => Err(()),
        }
    }
}

pub fn explore_maze_with(environment: &mut Environment, algorithm: &ExploreAlgorithm) {
    match algorithm {
        ExploreAlgorithm::WallFollowing => {
            follow_wall_explore(environment, *environment.maze.end.iter().next().unwrap());
        }

        ExploreAlgorithm::None => {
            environment.weighted_graph = environment.maze.convert_to_weighted_graph(None, true);
            return;
        }
    };
    environment.weighted_graph = environment
        .maze
        .convert_to_weighted_graph(Some(&environment.visited), true);
}
