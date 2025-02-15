use std::collections::HashSet;

use environment::environment::Environment;
use maze::maze::{Cell, Direction, Maze};
use maze_gen::maze_gen::{random_kruzkals_maze, random_wilson_maze};
use pyo3::prelude::*;
pub mod maze;
pub mod render;
pub mod environment;
pub mod maze_gen;
pub mod maze_solve;

#[pyfunction]
#[pyo3(signature=(width, height))]
fn init_environment_python(width: usize, height : usize) -> PyResult<Environment> {
    Ok(Environment::new(width, height))
    
}

#[pyfunction]
fn create_wilsons_maze(environment: &mut Environment) -> PyResult<()> {
    let walls_to_break_for_maze = random_wilson_maze(&mut environment.maze);
    environment.maze.break_walls_for_path(walls_to_break_for_maze);
    
    Ok(())
}
#[pyfunction]
fn create_kruzkals_maze(environment: &mut Environment) -> PyResult<()> {
    let walls_to_break_for_maze = random_kruzkals_maze(&mut environment.maze);
    environment.maze.break_walls_for_path(walls_to_break_for_maze);
    
    Ok(())
}

#[pymodule]
fn simulation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_environment_python, m)?)?;
    m.add_function(wrap_pyfunction!(create_wilsons_maze, m)?)?;
    m.add_function(wrap_pyfunction!(create_kruzkals_maze, m)?)?;
    m.add_class::<Cell>()?;
    m.add_class::<Maze>()?;
    m.add_class::<Direction>()?;
    m.add_class::<Environment>()?;
    Ok(())
}
