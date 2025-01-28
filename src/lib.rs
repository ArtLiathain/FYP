use maze::maze::{Cell, Maze};
use maze_logic::maze_logic::init_maze;
use pyo3::prelude::*;
pub mod maze;
pub mod maze_logic;

#[pyfunction]
fn init_maze_python(width: usize, height : usize) -> PyResult<Maze> {
    Ok(init_maze(width, height))
}
#[pyfunction]
fn return_steps(maze: &Maze) -> PyResult<usize> {
    Ok(maze.steps)
}


#[pymodule]
fn simulation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_maze_python, m)?)?;
    m.add_function(wrap_pyfunction!(return_steps, m)?)?;
    m.add_class::<Cell>()?;
    m.add_class::<Maze>()?;
    Ok(())
}
