pub mod python_bindings {
    use pyo3::{pyfunction, pymodule, types::{PyModule, PyModuleMethods}, wrap_pyfunction, Bound, PyResult};

    use crate::{environment::environment::Environment, maze::maze::Direction, maze_gen::maze_gen::{random_kruzkals_maze, random_wilson_maze}, python_bindings::environment_bindings::{Action, ActionResult, Info}};

    #[pyfunction]
    #[pyo3(signature=(width, height))]
    fn init_environment_python(width: usize, height: usize) -> PyResult<Environment> {
        Ok(Environment::new(width, height))
    }

    #[pyfunction]
    #[pyo3(signature=(direction))]
    fn create_action(direction: Direction) -> PyResult<Action> {
        Ok(Action { direction })
    }

    #[pyfunction]
    fn create_wilsons_maze(environment: &mut Environment) -> PyResult<()> {
        let walls_to_break_for_maze = random_wilson_maze(&mut environment.maze);
        environment
            .maze
            .break_walls_for_path(walls_to_break_for_maze);

        Ok(())
    }
    #[pyfunction]
    fn create_kruzkals_maze(environment: &mut Environment) -> PyResult<()> {
        let walls_to_break_for_maze = random_kruzkals_maze(&mut environment.maze);
        environment
            .maze
            .break_walls_for_path(walls_to_break_for_maze);

        Ok(())
    }

    #[pymodule]
    fn setup_python_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(init_environment_python, m)?)?;
        m.add_function(wrap_pyfunction!(create_wilsons_maze, m)?)?;
        m.add_function(wrap_pyfunction!(create_action, m)?)?;
        m.add_function(wrap_pyfunction!(create_kruzkals_maze, m)?)?;
        m.add_class::<Direction>()?;
        m.add_class::<Environment>()?;
        m.add_class::<Action>()?;
        m.add_class::<ActionResult>()?;
        m.add_class::<Info>()?;
        Ok(())
    }
}
