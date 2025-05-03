use std::collections::HashSet;

use macroquad::{color::{BLACK, GOLD, GREEN, LIGHTGRAY, PINK, RED, WHITE, YELLOW}, shapes::{draw_line, draw_rectangle}, text::draw_text, window::{clear_background, next_frame}};

use crate::{direction::Direction, environment::environment::{Coordinate, Environment}, maze::maze::Cell};


pub async fn render_maze(
    environment: &Environment,
    visited: &HashSet<Coordinate>,
    cell_size: f32,
    step: usize,
) {
    clear_background(BLACK);
    draw_maze(&environment, cell_size, visited, step, 0.0, 0.0).await;
    next_frame().await;
}
pub async fn draw_maze(
    environment: &Environment,
    cell_size: f32,
    visited: &HashSet<Coordinate>,
    step: usize,
    x_offset: f32,
    y_offset: f32,
) {
    let base_offset = 20.0;
    let current_run = environment.path_followed[step].1;
    draw_text(
        &format!(
            "Run {}, Currently: {}",
            current_run,
            if current_run
                >= environment
                    .config
                    .python_config
                    .mini_explore_runs_per_episode
                    - 1
            {
                "Solving"
            } else {
                "Exploring"
            }
        ),
        x_offset + base_offset,
        y_offset + base_offset - 5.0, // Adjust upward a little
        20.0,                         // Font size
        YELLOW,                       // Color (adjust as you like)
    );

    let mut path_visited = HashSet::new();
    let path_start_index = environment
        .path_followed
        .iter()
        .enumerate()
        .find(|(_, (_, run))| *run == current_run)
        .map(|(index, _)| index)
        .unwrap();
    for i in 0..(step - path_start_index) {
        path_visited.insert(environment.path_followed[i + path_start_index].0);
    }
    for row in &environment.maze.grid {
        for cell in row {
            draw_cell(
                cell,
                cell_size,
                base_offset,
                environment,
                visited,
                step,
                &path_visited,
                x_offset,
                y_offset,
            )
            .await;
        }
    }
}



pub fn draw_cell_walls(cell: &Cell, cell_size: f32, x: f32, y: f32, thickness: f32) {
    if cell.walls.contains(&Direction::North) {
        draw_line(x, y, x + cell_size, y, thickness, WHITE);
    }
    if cell.walls.contains(&Direction::East) {
        draw_line(
            x + cell_size,
            y,
            x + cell_size,
            y + cell_size,
            thickness,
            WHITE,
        );
    }
    if cell.walls.contains(&Direction::South) {
        draw_line(
            x,
            y + cell_size,
            x + cell_size,
            y + cell_size,
            thickness,
            WHITE,
        );
    }
    if cell.walls.contains(&Direction::West) {
        draw_line(x, y, x, y + cell_size, thickness, WHITE);
    }
}

pub async fn draw_cell(
    cell: &Cell,
    cell_size: f32,
    offset: f32,
    environment: &Environment,
    visited: &HashSet<Coordinate>,
    step: usize,
    path: &HashSet<Coordinate>,
    x_offset: f32,
    y_offset: f32,
) {
    let x = cell.x as f32 * cell_size + offset + x_offset;
    let y = cell.y as f32 * cell_size + offset + y_offset;
    let coordinates = (cell.x, cell.y);

    if cell.walls.len() == 4 {
        draw_rectangle(x, y, cell_size, cell_size, WHITE);
    } else {
        draw_rectangle(x, y, cell_size, cell_size, BLACK);
    }

    if visited.contains(&coordinates) {
        draw_rectangle(x, y, cell_size, cell_size, LIGHTGRAY);
    }

    if environment.maze.end.contains(&coordinates) {
        draw_rectangle(x, y, cell_size, cell_size, GOLD); // Change RED to any color you prefer
    }

    if coordinates == environment.maze.start {
        draw_rectangle(x, y, cell_size, cell_size, GREEN);
    }

    if path.contains(&coordinates) {
        draw_rectangle(x, y, cell_size, cell_size, PINK);
    }

    if environment.path_followed[step].0 == coordinates {
        draw_rectangle(x, y, cell_size, cell_size, RED);
    }
    // Draw the cell walls based on its directions
    draw_cell_walls(cell, cell_size, x, y, 1.0);
}