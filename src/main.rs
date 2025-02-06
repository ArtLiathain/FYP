use std::{
    thread::{self, sleep},
    time::Duration,
};

use maze::maze::{Direction, Maze};
use maze_logic::maze_logic::{
    init_maze, random_kruzkals_maze, random_wilson_maze, solve_maze_for_animated_dfs,
};
use render::render::render_maze;

pub mod maze;
pub mod maze_logic;
pub mod render;

#[macroquad::main("Maze Visualizer")]
async fn main() {
    let cell_size = 20.0;
    let mut maze = init_maze(25, 25);
    
    let walls_to_break_for_maze = random_kruzkals_maze(&mut maze);
    for i in 0..walls_to_break_for_maze.len() {
        Maze::break_walls_for_path_animated(&mut maze, &walls_to_break_for_maze, i);
        render_maze(&maze, cell_size).await;
        thread::sleep(Duration::from_millis(10));
    }
    
    let visited = solve_maze_for_animated_dfs(&mut maze);
    let mut step: usize = 0;
    thread::sleep(Duration::from_millis(1000));
    loop {
        if step >= visited.len() {
            break;
        }
        let current = visited[step];
        let mut backwards_steps = 0;
        while current.1 < maze.path.len() {
            maze.path.remove(&visited[step - backwards_steps].0);
            backwards_steps += 1;
        }
        maze.visited.insert(current.0);
        maze.path.insert(current.0);

        render_maze(&maze, cell_size).await;
        step += 1;
    }
    loop {
        render_maze(&maze, cell_size).await;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_vector_unvisited_nodes() {
//         let temp: Vec<(usize, usize)> = Vec::new();
//         assert_eq!(temp, random_maze(3, 4))
//     }
// }
