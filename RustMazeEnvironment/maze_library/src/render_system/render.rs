pub mod render {
    use crate::constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
    use crate::environment::environment::{Coordinate, Environment};
    use crate::render_system::render_coloured_mazes::draw_coloured_maze;
    use crate::render_system::render_maze::draw_maze;
    use macroquad::color::YELLOW;
    use macroquad::input::{is_key_pressed, KeyCode};
    use macroquad::text::draw_text;
    use macroquad::window::next_frame;
    use std::cmp::min;
    use std::collections::HashSet;
    use std::thread::sleep;
    use std::time::Duration;

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
        let full_episode = environments[0].maze.number_of_cells();
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
                    println!("Right pressed - skipping {} steps.", full_episode);
                    skip_steps = step + full_episode;
                    next_frame().await; // let the frame advance
                    sleep(Duration::from_millis(300));
                }
                if is_key_pressed(KeyCode::Up) {
                    println!("Up pressed - skipping {} steps.", full_episode * 4);
                    skip_steps = step + full_episode * 4;
                    next_frame().await; // let the frame advance
                    sleep(Duration::from_millis(300));
                }

                let mut screens_displayed = 0;
                if !coloured_heatmap {
                    draw_text(
                        &format!("Step {}", step),
                        10.0,
                        10.0,   // Adjust upward a little
                        20.0,   // Font size
                        YELLOW, // Color (adjust as you like)
                    );
                }

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
                                if environments[env_index].path_followed.len() > 5 {
                                    true
                                } else {
                                    false
                                },
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
                    sleep(Duration::from_millis(100));
                }
                step += 1;
            }
            if is_array_all_true(&array_of_complete) {
                sleep(Duration::from_millis(3000));
            }
        }
    }
}
