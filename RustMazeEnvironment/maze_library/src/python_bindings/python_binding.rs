pub mod python_bindings {

    use pyo3::{
        pyfunction, pymodule,
        types::{PyModule, PyModuleMethods},
        wrap_pyfunction, Bound, PyResult,
    };

    use crate::{
        direction::Direction,
        environment::environment::Environment,
        environment_config::{EnvConfig, PythonConfig},
        maze_gen::{kruzkals::random_kruzkals_maze, wilsons::random_wilson_maze},
        python_bindings::environment_bindings::{Action, ActionResult},
    };

    #[pyfunction]
    #[pyo3(signature=(width, height, allowed_revisits))]
    fn init_environment_python(
        width: usize,
        height: usize,
        allowed_revisits: usize,
    ) -> PyResult<Environment> {
        let config: EnvConfig = EnvConfig::new(width, height, PythonConfig { allowed_revisits });
        Ok(Environment::new(config))
    }

    #[pyfunction]
    #[pyo3(signature=(direction, run))]
    fn create_action(direction: usize, run: usize) -> PyResult<Action> {
        Ok(Action { direction, run })
    }

    #[pyfunction]
    #[pyo3(signature=(env))]
    fn print_weighted_graph(env: &Environment) -> PyResult<()> {
        println!("{:?}", env.weighted_graph);
        Ok(())
    }

    #[pyfunction]
    fn create_wilsons_maze(environment: &mut Environment) -> PyResult<()> {
        let walls_to_break_for_maze = random_wilson_maze(&mut environment.maze);
        environment
            .maze
            .break_walls_for_path(walls_to_break_for_maze);
        environment.weighted_graph = environment.maze.convert_to_weighted_graph(None, true);
        Ok(())
    }
    #[pyfunction]
    fn make_maze_imperfect(environment: &mut Environment) -> PyResult<()> {
        let walls_to_break = environment.maze.break_random_walls(15);
        environment.maze.break_walls_for_path(walls_to_break);
        environment.weighted_graph = environment.maze.convert_to_weighted_graph(None, true);
        Ok(())
    }
    #[pyfunction]
    fn create_kruzkals_maze(environment: &mut Environment) -> PyResult<()> {
        let walls_to_break_for_maze = random_kruzkals_maze(&mut environment.maze);
        environment
            .maze
            .break_walls_for_path(walls_to_break_for_maze);
        environment.weighted_graph = environment.maze.convert_to_weighted_graph(None, true);
        Ok(())
    }

    #[pymodule]
    fn maze_library(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(init_environment_python, m)?)?;
        m.add_function(wrap_pyfunction!(create_wilsons_maze, m)?)?;
        m.add_function(wrap_pyfunction!(create_action, m)?)?;
        m.add_function(wrap_pyfunction!(create_kruzkals_maze, m)?)?;
        m.add_function(wrap_pyfunction!(print_weighted_graph, m)?)?;
        m.add_function(wrap_pyfunction!(make_maze_imperfect, m)?)?;
        m.add_class::<Direction>()?;
        m.add_class::<Environment>()?;
        m.add_class::<Action>()?;
        m.add_class::<ActionResult>()?;
        Ok(())
    }
}
