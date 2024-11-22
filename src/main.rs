use std::hash::Hash;
use std::usize;
use std::{collections::HashSet, fmt};

use macroquad::color::{BLACK, GOLD, GREEN, WHITE};
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::window::{clear_background, next_frame};
use rand::Rng; // For random number generation

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cell {
    x: usize,
    y: usize,
    walls: HashSet<Direction>,
    end: bool,
    start: bool,
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>, // Tracks which cells are part of the maze
    start : (usize, usize),
    end : (usize, usize)

}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        Maze {
            width,
            height,
            start : (0, height-1),
            end : (width/2, height/2),
            grid: (0..width)
                .map(|x| {
                    (0..height)
                        .map(move |y| Cell {
                            x,
                            y,
                            walls: HashSet::from([
                                Direction::North,
                                Direction::South,
                                Direction::East,
                                Direction::West,
                            ]),
                            start: false,
                            end: false,
                        })
                        .collect::<Vec<Cell>>()
                })
                .collect::<Vec<Vec<Cell>>>(),
        }
    }

    fn set_end(&mut self, cell: (usize, usize)) {
        self.grid[cell.0][cell.1].end = true;
    }

    fn in_bounds(&self, cell: (usize, usize)) -> bool {
        cell.0 < self.width && cell.1 < self.height
    }

    fn get_starting_point(&self) -> (usize, usize) {
        self.start
    }
    fn get_end_point(&self) -> (usize, usize) {
        self.end
    }



    fn set_starting_point(&mut self, coordinates: (usize, usize), delete_wall: Option<&Direction>) {
        let cell = &mut self.grid[coordinates.0][coordinates.1];
        if let Some(wall) = delete_wall {
            cell.walls.remove(wall);
        }
        cell.start = true;
    }

    fn break_walls_for_path(&mut self, path: Vec<((usize, usize), Direction)>) {
        for i in 0..path.len() - 1 {
            let current_cell = path[i].0;
            let next_cell = path[i + 1].0;
            let direction = path[i].1;
            self.grid[next_cell.0][next_cell.1]
                .walls
                .remove(&Direction::opposite_direction(&direction));
            self.grid[current_cell.0][current_cell.1]
                .walls
                .remove(&direction);

            // println!(
            //     "Wall removed Current cell {} {} {} {:?}",
            //     current_cell.0,
            //     current_cell.1,
            //     direction,
            //     self.grid[current_cell.0][current_cell.1].walls
            // );
            // println!(
            //     "Next cell {} {} {} {:?}",
            //     next_cell.0,
            //     next_cell.1,
            //     Direction::opposite_direction(&direction),
            //     self.grid[next_cell.0][next_cell.1].walls
            // );
        }
    }
}
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction_str = match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        };
        write!(f, "{}", direction_str)
    }
}

impl Direction {
    fn random() -> Direction {
        match rand::thread_rng().gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        }
    }

    fn move_from(&self, coordinates: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (coordinates.0, coordinates.1.saturating_sub(1)),
            Direction::South => (coordinates.0, coordinates.1 + 1),
            Direction::East => (coordinates.0 + 1, coordinates.1),
            Direction::West => (coordinates.0.saturating_sub(1), coordinates.1),
        }
    }

    fn opposite_direction(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

async fn draw_maze(maze: &Maze, cell_size: f32) {
    clear_background(BLACK);
    let offset = 10.0;
    for row in &maze.grid {
        for cell in row {
            // println!("Cell: {} {} {:?}", cell.x, cell.y, cell.walls);
            draw_cell(cell, cell_size, offset).await;
        }
    }
}

async fn draw_cell(cell: &Cell, cell_size: f32, offset: f32) {
    let x = cell.x as f32 * cell_size + offset;
    let y = cell.y as f32 * cell_size + offset;

    let thickness = 1.0;
    if cell.end {
        draw_rectangle(x, y, cell_size, cell_size, GOLD); // Change RED to any color you prefer
    }

    if cell.start {
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

fn random_maze(width: usize, height: usize) -> Maze {
    let mut maze: Maze = Maze::new(width, height);
    let mut unvisited_nodes: Vec<(usize, usize)> = (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .collect();

    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    let end_coordinate = (width / 2, height / 2);
    maze.set_end((width / 2, height / 2));
    maze.set_starting_point(
        unvisited_nodes[(height - 1)],
        Some((&Direction::South)),
    );
    let mut current = unvisited_nodes.remove((height - 1));
    while !unvisited_nodes.is_empty() {
        let mut new_path: Vec<((usize, usize), Direction)> = Vec::new();
        let mut new_coordinates: (usize, usize) = (0, 0);
        loop {
            let direction = Direction::random();
            // println!("COORDINATES {} {} {}", current.0, current.1, direction);
            // println!("Current path: {:?}", new_path);

            new_coordinates = Direction::move_from(&direction, &current);

            if !Maze::in_bounds(&maze, new_coordinates) {
                // println!("OUT OF BOUNDS {} {}", new_coordinates.0, new_coordinates.1);
                continue;
            }
            new_path.push((current, direction));

            let match_index = new_path
                .iter()
                .position(|(coordinates, _)| *coordinates == new_coordinates);

            if let Some(index) = match_index {
                // Truncate the vector at the found index (inclusive)
                // println!("Path before truncate: {:?}", new_path);
                new_path.truncate(index + 1); // Keeps the match and deletes after
                                              // println!("Path after truncate: {:?}", new_path);
                current = new_path.remove(new_path.len() - 1).0;
                continue;
            }

            if end_coordinate == new_coordinates || visited_nodes.contains(&new_coordinates) {
                // println!("Unvisited nodes: {:?}", unvisited_nodes);
                // println!("Visited nodes: {:?}", visited_nodes);
                new_path.push((new_coordinates, direction));

                break;
            }
            current = new_coordinates;
        }
        // println!("Current path: {:?}", new_path);
        Maze::break_walls_for_path(&mut maze, new_path.clone());
        for (coords, _) in new_path {
            visited_nodes.insert(coords);
            unvisited_nodes.retain(|&coord| coord != coords);
        }
        if (!unvisited_nodes.is_empty()) {
            current =
                unvisited_nodes.remove(rand::thread_rng().gen_range(0..unvisited_nodes.len()));
        }
    }
    let mut path = Vec::new();
    loop {
        let direction = Direction::random();
        let new_coordinates = Direction::move_from(&direction, &current);
        if !Maze::in_bounds(&maze, new_coordinates) || new_coordinates == current {
            // println!("OUT OF BOUNDS {} {}", new_coordinates.0, new_coordinates.1);
            continue;
        }
        path.push((current, direction));
        path.push((new_coordinates, direction));

        break;
    }
    Maze::break_walls_for_path(&mut maze, path);

    maze
}

fn solve_maze(maze : &Maze) -> Vec<(usize, usize)> {
    let mut current = maze.get_starting_point();
    let end = maze.get_end_point();
    let path : Vec<(usize, usize, bool)> = Vec::new();
    loop {
        
    }
    Vec::new()
}


#[macroquad::main("Maze Visualizer")]
async fn main() {
    let maze = random_maze(10, 10);
    let cell_size = 20.0;

    // Game loop
    loop {
        draw_maze(&maze, cell_size).await;
        next_frame().await;

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
