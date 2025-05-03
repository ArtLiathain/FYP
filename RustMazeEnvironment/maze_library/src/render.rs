pub mod render {
    use crate::constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
    use crate::direction::Direction;
    use crate::environment::environment::{Coordinate, Environment};
    use crate::maze::maze::Cell;
    use macroquad::color::{
        Color, BLACK, DARKPURPLE, GOLD, GREEN, LIGHTGRAY, PINK, RED, WHITE, YELLOW,
    };
    use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
    use macroquad::shapes::{draw_line, draw_rectangle};
    use macroquad::text::draw_text;
    use macroquad::window::{clear_background, next_frame};
    use rand::{rng, Rng, SeedableRng};
    use std::cmp::min;
    use std::collections::{HashMap, HashSet};
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

    fn draw_cell_walls(cell: &Cell, cell_size: f32, x: f32, y: f32, thickness: f32) {
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

    pub async fn render_mazes(
        environments: Vec<Environment>,
        cell_size: f32,
        coloured_heatmap: bool,
    ) {
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
            let mut skip_steps = 0;
            while !is_array_all_true(&array_of_complete) {
                if is_key_pressed(KeyCode::Space) {
                    println!("Space pressed - skipping current maze batch.");
                    next_frame().await; // let the frame advance
                    sleep(Duration::from_millis(300));
                    break; // only break this batch loop, continue to next one
                }
                if is_key_pressed(KeyCode::Right) {
                    println!("Right pressed - skipping 500 steps.");
                    skip_steps = step + 500;
                    next_frame().await; // let the frame advance
                    sleep(Duration::from_millis(300));
                }
                if is_key_pressed(KeyCode::Up) {
                    println!("Up pressed - skipping 2000 steps.");
                    skip_steps = step + 2000;
                    next_frame().await; // let the frame advance
                    sleep(Duration::from_millis(300));
                }

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

                        if coloured_heatmap {
                            draw_coloured_maze(
                                &environments[env_index],
                                cell_size,
                                cell_size * (col * (environments[0].maze.width + 2)) as f32,
                                cell_size * (row * (environments[0].maze.height + 2)) as f32,
                                &environments[env_index].overall_visited,
                            )
                            .await;
                        } else {
                            let step_to_use =
                                min(step, environments[env_index].path_followed.len() - 1);
                            if step_to_use >= environments[env_index].path_followed.len() - 1 {
                                array_of_complete[idx] = true;
                            }
                            if skip_steps > step {
                                visited_nodes[idx]
                                    .insert(environments[env_index].path_followed[step_to_use].0);
                                continue;
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
                        }
                        screens_displayed += 1;

                        if screens_displayed >= amount_of_screens {
                            break;
                        }
                    }
                    if screens_displayed >= amount_of_screens {
                        break;
                    }
                }
                if skip_steps < step {
                    next_frame().await;
                    sleep(Duration::from_millis(50));
                }
                step += 1;
            }
            if is_array_all_true(&array_of_complete) {
                sleep(Duration::from_millis(3000));
            }
        }
    }
}
