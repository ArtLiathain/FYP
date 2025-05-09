use std::collections::HashMap;

use num_traits::ToPrimitive;
use pyo3::pyclass;
use serde::{Deserialize, Serialize};

use crate::{
    environment::environment::{calcualte_score_for_coordinate_vector, Coordinate, Environment},
    solving_algorithms::dijkstra::dijkstra_solve,
};

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportCard {
    #[pyo3(get)]
    pub total_steps: usize,
    #[pyo3(get)]
    pub average_run_score: f32,
    #[pyo3(get)]
    pub average_path_length: f32,
    #[pyo3(get)]
    pub full_turns_done: f32,
    #[pyo3(get)]
    pub success_rate_in_exploitation: f32,
    #[pyo3(get)]
    pub total_percentage_explored: f32,
    #[pyo3(get)]
    pub dijkstra_shortest_path_score: usize,
    #[pyo3(get)]
    pub walls_hit: f32,
    #[pyo3(get)]
    pub percentage_visited: f32,
    #[pyo3(get)]
    pub average_visits: f32,
}

fn calculate_run_visited(
    env: &Environment,
    visited_map: &mut HashMap<Coordinate, usize>,
    run_to_score: usize,
) -> (f32, f32) {
    let filtered_path: Vec<Coordinate> = env
        .path_followed
        .iter()
        .filter(|(_, run)| *run == run_to_score)
        .map(|(coord, _)| *coord)
        .collect();
    for coordinate in filtered_path {
        *visited_map.entry(coordinate).or_insert(1) += 1;
    }

    (
        visited_map
            .iter()
            .map(|(_, value)| *value as f32)
            .sum::<f32>()
            / visited_map.len() as f32,
        visited_map.len() as f32 / env.maze.number_of_cells() as f32,
    )
}

fn average<T: ToPrimitive>(nums: &[T]) -> f32 {
    let sum: f32 = nums.iter().filter_map(|x| x.to_f32()).sum();
    sum / nums.len() as f32
}

impl Environment {
    pub fn generate_report_card(&self) -> ReportCard {
        let mut path_lengths = vec![];
        let mut hit_counts = vec![];
        let mut reverse_counts = vec![];
        let mut exits_found = vec![];
        let mut exploit_runs = vec![];
        let mut visited_tracker = HashMap::new();
        let mut percentage_visited = vec![];
        let mut average_visited = vec![];
        for i in 0..self.config.python_config.mini_explore_runs_per_episode {
            let (average, percentage) = calculate_run_visited(self, &mut visited_tracker, i);
            percentage_visited.push(percentage);
            average_visited.push(average);
        }

        for i in self.config.python_config.mini_explore_runs_per_episode..self.get_current_run() {
            let (total_run_score, hit_count, reverse_count, average_path_length, found_exit) =
                self.calculate_run_score(i);
            path_lengths.push(average_path_length);
            hit_counts.push(hit_count);
            exits_found.push(if found_exit { 1 } else { 0 });
            reverse_counts.push(reverse_count);
            if found_exit {
                exploit_runs.push(total_run_score);
            }
        }
        let (score, _, _, _) = calcualte_score_for_coordinate_vector(
            &dijkstra_solve(self, self.maze.start, *self.maze.end.iter().next().unwrap()),
            &self.weighted_graph,
        );
        ReportCard {
            total_steps: self.total_steps,
            average_path_length: average(&path_lengths),
            full_turns_done: average(&reverse_counts),
            success_rate_in_exploitation: average(&exits_found),
            total_percentage_explored: self.overall_visited.len() as f32
                / self.maze.number_of_cells() as f32,
            dijkstra_shortest_path_score: score,
            walls_hit: average(&hit_counts),
            average_run_score: average(&exploit_runs),
            percentage_visited: average(&percentage_visited),
            average_visits: average(&average_visited),
        }
    }
}
