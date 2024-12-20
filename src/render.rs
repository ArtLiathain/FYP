pub mod render {
    use crate::maze::maze::{Cell, Direction, Maze};
    use macroquad::color::{BLACK, BLUE, GOLD, GREEN, PINK, WHITE};
    use macroquad::shapes::{draw_line, draw_rectangle};
    use macroquad::window::{clear_background, next_frame};

    pub async fn draw_maze(maze: &Maze, cell_size: f32) {
        clear_background(BLACK);
        let offset = 10.0;
        for row in &maze.grid {
            for cell in row {
                // println!("Cell: {} {} {:?}", cell.x, cell.y, cell.walls);
                draw_cell(cell, cell_size, offset, maze).await;
            }
        }
    }

    pub async fn draw_cell(cell: &Cell, cell_size: f32, offset: f32, maze: &Maze) {
        let x = cell.x as f32 * cell_size + offset;
        let y = cell.y as f32 * cell_size + offset;
        let coordinates = (cell.x, cell.y);
        let thickness = 1.0;

        if maze.visited.contains(&coordinates) {
            draw_rectangle(x, y, cell_size, cell_size, BLUE);
        }

        if maze.path.contains(&coordinates) {
            draw_rectangle(x, y, cell_size, cell_size, PINK);
        }

        if coordinates == maze.end {
            draw_rectangle(x, y, cell_size, cell_size, GOLD); // Change RED to any color you prefer
        }

        if coordinates == maze.start {
            draw_rectangle(x, y, cell_size, cell_size, GREEN);
        }
        // Draw the cell walls based on its directions
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

    pub async fn render_maze(maze: &Maze, cell_size: f32) {
        draw_maze(&maze, cell_size).await;
        next_frame().await;
    }
}
