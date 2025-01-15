pub mod maze {
    use std::{collections::HashSet, fmt};

    use rand::Rng;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Cell {
        pub x: usize,
        pub y: usize,
        pub walls: HashSet<Direction>,
    }

    #[derive(Debug)]
    pub struct Maze {
        pub width: usize,
        pub height: usize,
        pub grid: Vec<Vec<Cell>>,
        pub path: HashSet<(usize, usize)>,
        pub visited: HashSet<(usize, usize)>,
        pub start: (usize, usize),
        pub end: (usize, usize),
    }

    impl Maze {
        pub fn new(width: usize, height: usize) -> Self {
            Maze {
                width,
                height,
                start: (0, height - 1),
                end: (width / 2, height / 2),
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
                            })
                            .collect::<Vec<Cell>>()
                    })
                    .collect::<Vec<Vec<Cell>>>(),
                path: HashSet::new(),
                visited: HashSet::new(),
            }
        }

        pub fn set_end(&mut self, cell: (usize, usize)) {
            self.end = cell;
        }
        pub fn in_bounds(&self, cell: (usize, usize)) -> bool {
            cell.0 < self.width && cell.1 < self.height
        }

        pub fn get_starting_point(&self) -> (usize, usize) {
            self.start
        }
        pub fn get_end_point(&self) -> (usize, usize) {
            self.end
        }

        pub fn set_starting_point(
            &mut self,
            coordinates: (usize, usize),
            delete_wall: Option<&Direction>,
        ) {
            let cell = &mut self.grid[coordinates.0][coordinates.1];
            if let Some(wall) = delete_wall {
                cell.walls.remove(wall);
            }
        }

        pub fn break_walls_for_path(&mut self, path: Vec<((usize, usize), Direction)>) {
            for i in 0..path.len() - 1 {
                let current_cell = path[i].0;
                let next_cell = Direction::move_from(&path[i].1, &path[i].0);
                let direction = path[i].1;
                self.grid[next_cell.0][next_cell.1]
                    .walls
                    .remove(&Direction::opposite_direction(&direction));
                self.grid[current_cell.0][current_cell.1]
                    .walls
                    .remove(&direction);
            }
        }
        pub fn break_walls_for_path_animated(&mut self, path: &Vec<((usize, usize), Direction)>, index : usize) {
                let current_cell = path[index].0;
                let next_cell = Direction::move_from(&path[index].1, &path[index].0);
                let direction = path[index].1;
                self.grid[next_cell.0][next_cell.1]
                    .walls
                    .remove(&Direction::opposite_direction(&direction));
                self.grid[current_cell.0][current_cell.1]
                    .walls
                    .remove(&direction);
        }
    }

    #[repr(usize)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
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
        pub fn random() -> Direction {
            match rand::thread_rng().gen_range(0..4) {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                _ => Direction::West,
            }
        }

        pub fn move_from(&self, coordinates: &(usize, usize)) -> (usize, usize) {
            match self {
                Direction::North => (coordinates.0, coordinates.1.saturating_sub(1)),
                Direction::South => (coordinates.0, coordinates.1 + 1),
                Direction::East => (coordinates.0 + 1, coordinates.1),
                Direction::West => (coordinates.0.saturating_sub(1), coordinates.1),
            }
        }

        pub fn opposite_direction(&self) -> Direction {
            match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            }
        }
    }
}
