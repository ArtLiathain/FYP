use std::collections::HashSet;

use maze_library::{direction::Direction, environment::environment::Environment};

pub fn follow_wall_explore(env: &mut Environment) {
    let start = env.maze.start;
    let end = env.maze.end;
    let mut previous: Option<Direction> = None;
    let mut has_not_reached_end = true;
    while env.current_location != start || has_not_reached_end {
        if env.current_location == end {
            has_not_reached_end = false;
        }
        let mut directions: HashSet<Direction> =
            env.available_paths().iter().map(|(dir, _)| *dir).collect();
        if previous.is_some() {
            directions.remove(&previous.unwrap().opposite_direction());
        }
        if directions.len() == 0 {
            env.move_from_current(&previous.unwrap().opposite_direction());
            previous = Some(previous.unwrap().opposite_direction());
            continue;
        }
        for direction in [
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ] {
            let dir = direction.relative_direction(&previous.unwrap_or(Direction::North));
            if directions.contains(&dir) {
                env.move_from_current(&dir);
                previous = Some(dir);
                break;
            }
        }
    }
}
