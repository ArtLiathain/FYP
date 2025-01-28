use maze::maze::{Cell, Direction, Maze};
use maze_logic::maze_logic::{init_maze, random_wilson_maze};
use pyo3::prelude::*;
pub mod maze;
pub mod maze_logic;

#[pyfunction]
fn init_maze_python(width: usize, height : usize) -> PyResult<Maze> {
    Ok(init_maze(width, height))
}

#[pyfunction]
fn create_wilsons_maze(maze: &mut Maze) -> PyResult<()> {
    let walls_to_break_for_maze = random_wilson_maze(maze);
    maze.break_walls_for_path(walls_to_break_for_maze);
    
    Ok(())
}

#[pymodule]
fn simulation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_maze_python, m)?)?;
    m.add_function(wrap_pyfunction!(create_wilsons_maze, m)?)?;
    m.add_class::<Cell>()?;
    m.add_class::<Maze>()?;
    m.add_class::<Direction>()?;
    Ok(())
}
