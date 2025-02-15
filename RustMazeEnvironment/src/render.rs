pub mod render {
    use std::collections::HashSet;
    use crate::environment::environment::{Coordinate, Environment};
    use crate::maze::maze::{Cell, Direction};
    use macroquad::color::{BLACK, GOLD, GREEN, LIGHTGRAY, RED, WHITE};
    use macroquad::shapes::{draw_line, draw_rectangle};
    use macroquad::window::{clear_background, next_frame};

    pub async fn draw_maze(
        environment: &Environment,
        cell_size: f32,
        visited: &HashSet<Coordinate>,
        step: usize,
    ) {
        clear_background(BLACK);
        let offset = 10.0;
        for row in &environment.maze.grid {
            for cell in row {
                // println!("Cell: {} {} {:?}", cell.x, cell.y, cell.walls);
                draw_cell(cell, cell_size, offset, environment, visited, step).await;
            }
        }
    }

    pub async fn draw_cell(
        cell: &Cell,
        cell_size: f32,
        offset: f32,
        environment: &Environment,
        visited: &HashSet<Coordinate>,
        step: usize,
    ) {
        let x = cell.x as f32 * cell_size + offset;
        let y = cell.y as f32 * cell_size + offset;
        let coordinates = (cell.x, cell.y);
        let thickness = 1.0;

        if cell.walls.len() == 4 {
            draw_rectangle(x, y, cell_size, cell_size, BLACK);
        } else {
            draw_rectangle(x, y, cell_size, cell_size, WHITE);
        }

        if visited.contains(&coordinates) {
            draw_rectangle(x, y, cell_size, cell_size, LIGHTGRAY);
        }

        if environment.path_followed[step] == coordinates {
            draw_rectangle(x, y, cell_size, cell_size, RED);
        }

        if coordinates == environment.maze.end {
            draw_rectangle(x, y, cell_size, cell_size, GOLD); // Change RED to any color you prefer
        }

        if coordinates == environment.maze.start {
            draw_rectangle(x, y, cell_size, cell_size, GREEN);
        }
        // Draw the cell walls based on its directions
        if cell.walls.contains(&Direction::North) {
            draw_line(x, y, x + cell_size, y, thickness, BLACK);
        }
        if cell.walls.contains(&Direction::East) {
            draw_line(
                x + cell_size,
                y,
                x + cell_size,
                y + cell_size,
                thickness,
                BLACK,
            );
        }
        if cell.walls.contains(&Direction::South) {
            draw_line(
                x,
                y + cell_size,
                x + cell_size,
                y + cell_size,
                thickness,
                BLACK,
            );
        }
        if cell.walls.contains(&Direction::West) {
            draw_line(x, y, x, y + cell_size, thickness, BLACK);
        }
    }

    pub async fn render_maze(
        environment: &Environment,
        visited: &HashSet<Coordinate>,
        cell_size: f32,
        step: usize,
    ) {
        draw_maze(&environment, cell_size, visited, step).await;
        next_frame().await;
    }
}
