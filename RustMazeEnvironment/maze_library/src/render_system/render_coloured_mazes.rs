use std::collections::HashMap;

use macroquad::{color::{Color, BLACK, DARKPURPLE}, shapes::draw_rectangle};

use crate::{environment::environment::{Coordinate, Environment}, maze::maze::Cell};

use super::render_maze::draw_cell_walls;

pub async fn draw_coloured_maze(
    environment: &Environment,
    cell_size: f32,
    x_offset: f32,
    y_offset: f32,
    path_map: &HashMap<Coordinate, usize>,
) {
    let base_offset = 10.0;
    let max_steps = path_map.values().max().unwrap_or(&100);
    for row in &environment.maze.grid {
        for cell in row {
            draw_cell_coloured(
                cell,
                cell_size,
                base_offset,
                x_offset,
                y_offset,
                path_map,
                *max_steps,
                Color::from_rgba(30, 144, 255, 255),
                environment,
            )
            .await;
        }
    }
}

async fn draw_cell_coloured(
    cell: &Cell,
    cell_size: f32,
    offset: f32,
    x_offset: f32,
    y_offset: f32,
    path_map: &HashMap<Coordinate, usize>,
    max_steps: usize,
    random_colour: Color,
    environment: &Environment,
) {
    let x = cell.x as f32 * cell_size + offset + x_offset;
    let y = cell.y as f32 * cell_size + offset + y_offset;
    let coordinates = (cell.x, cell.y);
    let base_color = if let Some(steps) = path_map.get(&coordinates) {
        // Clamp steps to a maximum for color normalization
        let normalized = (*steps as f32 / max_steps as f32).min(1.0);
        let brightness = 1.0 - normalized;
        Color::new(
            random_colour.r * brightness,
            random_colour.g * brightness,
            random_colour.b * brightness,
            1.0,
        )
    } else {
        random_colour
    };
    if cell.walls.len() == 4 {
        draw_rectangle(x, y, cell_size, cell_size, BLACK);
    } else {
        draw_rectangle(x, y, cell_size, cell_size, base_color);
    }

    if environment.maze.end.contains(&coordinates) {
        draw_rectangle(x, y, cell_size, cell_size, DARKPURPLE); // Change RED to any color you prefer
    }

    if coordinates == environment.maze.start {
        draw_rectangle(x, y, cell_size, cell_size, DARKPURPLE);
    }

    draw_cell_walls(cell, cell_size, x, y, 1.0);
}