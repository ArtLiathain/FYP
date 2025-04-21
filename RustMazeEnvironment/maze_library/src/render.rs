pub mod render {
    use crate::constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
    use crate::direction::Direction;
    use crate::environment::environment::{Coordinate, Environment};
    use crate::maze::maze::Cell;
    use macroquad::color::{BLACK, GOLD, GREEN, LIGHTGRAY, PINK, RED, WHITE};
    use macroquad::shapes::{draw_line, draw_rectangle};
    use macroquad::window::{clear_background, next_frame};
    use std::cmp::min;
    use std::collections::HashSet;
    use std::thread::sleep;
    use std::time::Duration;

    pub async fn draw_maze(
        environment: &Environment,
        cell_size: f32,
        visited: &HashSet<Coordinate>,
        step: usize,
        x_offset: f32,
        y_offset: f32,
    ) {
        let base_offset = 10.0;
        let current_run = environment.get_current_run();
        let mut path_visited = HashSet::new();
        let path_start_index = environment
            .path_followed
            .iter()
            .enumerate()
            .find(|(_, (_, run))| *run == current_run)
            .map(|(index, _)| index)
            .unwrap();
        if step > path_start_index {
            for i in 0..(step - path_start_index) {
                path_visited.insert(environment.path_followed[i + path_start_index].0);
            }
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
        let thickness = 1.0;

        if cell.walls.len() == 4 {
            draw_rectangle(x, y, cell_size, cell_size, BLACK);
        } else {
            draw_rectangle(x, y, cell_size, cell_size, WHITE);
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
        clear_background(BLACK);
        draw_maze(&environment, cell_size, visited, step, 0.0, 0.0).await;
        next_frame().await;
    }

    fn calculate_number_of_potential_screens(
        screen_size: (usize, usize),
        maze_size: (usize, usize),
    ) -> (usize, usize) {
        (screen_size.1 / maze_size.1, screen_size.0 / maze_size.0)
    }

    fn make_visited_sets_array(amount: usize) -> Vec<HashSet<Coordinate>> {
        vec![HashSet::new(); amount]
    }

    fn is_array_all_true(bools: &[bool]) -> bool {
        bools.iter().all(|&b| b)
    }

    pub async fn render_mazes(environments: Vec<Environment>, cell_size: f32) {
        println!("RUNNNING MAZES {}", environments.len());
        let (rows, columns) = calculate_number_of_potential_screens(
            (WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize),
            (
                (environments[0].maze.width + 2) * cell_size as usize,
                (environments[0].maze.height + 2) * cell_size as usize,
            ),
        );

        for environments_index in (0..environments.len()).step_by(rows * columns) {
            let amount_of_screens = min(rows * columns, environments.len() - environments_index);
            println!(
                "\n--- Rendering Batch ---\nEnvironments Index: {}\nAmount of Screens: {}\n",
                environments_index, amount_of_screens
            );

            let mut visited_nodes = make_visited_sets_array(amount_of_screens);
            let mut array_of_complete = vec![false; amount_of_screens];
            let mut step = 0;

            while !is_array_all_true(&array_of_complete) {
                let mut screens_displayed = 0;

                for row in 0..rows {
                    for col in 0..columns {
                        let idx = row * columns + col;
                        if idx >= amount_of_screens {
                            continue;
                        }

                        let env_index = environments_index + idx;
                        if env_index >= environments.len() {
                            continue;
                        }

                        let step_to_use =
                            min(step, environments[env_index].path_followed.len() - 1);
                        if step_to_use >= environments[env_index].path_followed.len() - 1 {
                            array_of_complete[idx] = true;
                        }

                        draw_maze(
                            &environments[env_index],
                            cell_size,
                            &visited_nodes[idx],
                            step_to_use,
                            cell_size * (col * (environments[0].maze.width + 2)) as f32,
                            cell_size * (row * (environments[0].maze.height + 2)) as f32,
                        )
                        .await;
                        visited_nodes[idx]
                            .insert(environments[env_index].path_followed[step_to_use].0);
                        screens_displayed += 1;
                        if screens_displayed >= amount_of_screens {
                            break;
                        }
                    }
                    if screens_displayed >= amount_of_screens {
                        break;
                    }
                }
                next_frame().await;
                sleep(Duration::from_millis(50));
                step += 1;
            }
            sleep(Duration::from_millis(3000));

        }
    }
}
