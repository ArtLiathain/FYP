use std::fmt;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[repr(usize)]
#[cfg_attr(feature = "python", pyo3::pyclass(eq, eq_int))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[cfg_attr(feature = "python", pyo3::pymethods)]
impl Direction {
    pub fn __hash__(&self) -> u64 {
        *self as u64
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

impl Direction {
    pub fn random() -> Direction {
        match rand::rng().random_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        }
    }
}