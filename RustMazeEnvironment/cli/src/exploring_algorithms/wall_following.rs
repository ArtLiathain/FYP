use std::collections::HashSet;

use maze_library::{
    direction::Direction,
    environment::environment::{Coordinate, Environment},
};

pub fn follow_wall_explore(env: &mut Environment, end: Coordinate) {
    let start = env.maze.start;
    let mut has_not_reached_end = true;
    let run = env.get_current_run() + 1;
    while env.current_location != start || has_not_reached_end {
        if env.current_location == end {
            has_not_reached_end = false;
        }
        let mut directions: HashSet<Direction> =
            env.available_paths().iter().map(|(dir, _)| *dir).collect();
        if env.previous_direction.is_some() {
            directions.remove(&env.previous_direction.unwrap().opposite_direction());
        }
        if directions.len() == 0 {
            env.move_from_current(&env.previous_direction.unwrap().opposite_direction(), run);
            continue;
        }
        for direction in [
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ] {
            let dir =
                direction.relative_direction(&env.previous_direction.unwrap_or(Direction::North));
            if directions.contains(&dir) {
                env.move_from_current(&dir, run);
                break;
            }
        }
    }
}
