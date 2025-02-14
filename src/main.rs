use std::{
    env, fs::File, io::Read, thread::{self, sleep}, time::Duration
};

use macroquad::file;
use maze::maze::{ Direction, Maze};
use maze_logic::maze_logic::{
    init_maze, random_kruzkals_maze, random_wilson_maze, solve_maze_for_animated_dfs,
};
use render::render::render_maze;

pub mod maze;
pub mod maze_logic;
pub mod render;

#[macroquad::main("Maze Visualizer")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let cell_size = 20.0;
    let mut maze = init_maze(25, 25);
    let walls_to_break_for_maze: Vec<((usize, usize), Direction)> = random_kruzkals_maze(&mut maze);
    maze.break_walls_for_path(walls_to_break_for_maze);
    // for i in 0..walls_to_break_for_maze.len() {
    //     maze.break_wall_for_path( &walls_to_break_for_maze, i);
    //     render_maze(&maze, cell_size).await;
    //     sleep(Duration::from_millis(10));
    // }

    let visited = solve_maze_for_animated_dfs(&mut maze);
    thread::sleep(Duration::from_millis(1000));
    render_maze_loop(&mut maze, visited, cell_size).await;
}

async fn render_maze_loop(maze: &mut Maze, visited: Vec<((usize, usize), usize)>, cell_size: f32) {
    let mut step: usize = 0;
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

fn read_maze_from_file(filename : &str) -> Maze {
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
    Maze::from_json(&contents).unwrap()
}

fn select_maze_gen_algorithm(algorithm : &str, maze : &mut Maze) -> Vec<((usize, usize), Direction)> {
    let algorithm_lower = algorithm.to_lowercase();
    if algorithm_lower == "wilson" {
        return random_wilson_maze(maze)
    }
    else if algorithm_lower == "kruzkal" {
        return random_kruzkals_maze(maze)
    }
    random_kruzkals_maze(maze)
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
