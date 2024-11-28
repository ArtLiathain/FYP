
use maze_logic::maze_logic::{random_wilson_maze, solve_maze_for_animated_dfs};
use render::render::render_maze;

pub mod maze;
pub mod maze_logic;
pub mod render;

#[macroquad::main("Maze Visualizer")]
async fn main() {
    let cell_size = 20.0;
    let mut maze = random_wilson_maze(40, 20);
    let visited = solve_maze_for_animated_dfs(&maze);
    // let mut new_path = Vec::new();
    let mut step: usize = 0;
    loop {
        if step >= visited.len() {
            break;
        }
        let current = visited[step];
        let mut backwards_steps = 0;
        while current.1 < maze.path.len() {
            // println!("visited {:?}", maze.visited);
            // println!("path {:?}", maze.path);
            // println!("step {:?}", step - backwards_steps);
            maze.path.remove(&visited[step - backwards_steps].0);
            backwards_steps += 1;
            render_maze(&maze, cell_size).await;
        }
        maze.visited.insert(current.0);
        maze.path.insert(current.0);
        render_maze(&maze, cell_size).await;
        step += 1;
        // sleep(Duration::new(0, 1000000));
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
