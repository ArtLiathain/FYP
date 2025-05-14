pub mod python_bindings {
    use std::str::FromStr;

    use pyo3::{
        pyfunction, pymodule,
        types::{PyModule, PyModuleMethods},
        wrap_pyfunction, Bound, PyResult,
    };
    use strum::IntoEnumIterator;

    use crate::{
        direction::Direction,
        environment::environment::Environment,
        environment_config::{EnvConfig, PythonConfig},
        maze_gen::maze_gen_handler::{select_maze_algorithm, MazeType, MazeTypeIter},
        python_bindings::{
            environment_bindings::{Action, ActionResult},
            report_card::ReportCard,
        },
    };

    #[pyfunction(
        signature = (width, height,gen_algorithm=String::from("kruzkals"), allowed_revisits=50, use_sparse_rewards=false, use_weighted_graph=true, rng_seed=None, mini_exploit_runs_per_episode=2, mini_explore_runs_per_episode=2, exploration_steps=None),
        text_signature = "(width, height,gen_algorithm='kruzkals', allowed_revisits=50, use_sparse_rewards=False,use_weighted_graph=True, rng_seed=None, mini_exploit_runs_per_episode=2, mini_explore_runs_per_episode=2, exploration_steps=None)"
    )]
    fn init_environment(
        width: usize,
        height: usize,
        gen_algorithm: String,
        allowed_revisits: usize,
        use_sparse_rewards: bool,
        use_weighted_graph: bool,
        rng_seed: Option<u64>,
        mini_exploit_runs_per_episode: usize,
        mini_explore_runs_per_episode: usize,
        exploration_steps: Option<usize>,
    ) -> PyResult<Environment> {
        let gen_algo = MazeType::from_str(&gen_algorithm).unwrap_or(MazeType::Kruzkals);

        let config: EnvConfig = EnvConfig::new(
            width,
            height,
            PythonConfig {
                allowed_revisits,
                use_sparse_rewards,
                mini_exploit_runs_per_episode,
                mini_explore_runs_per_episode,
                exploration_steps: if exploration_steps.is_some() {
                    exploration_steps.unwrap()
                } else {
                    width * height
                },
                generated_maze_type: gen_algo.clone(),
            },
        );
        let mut env = Environment::new(config);
        let walls = select_maze_algorithm(&env.maze, rng_seed, &gen_algo);
        env.maze.break_walls_for_path(walls);
        env.weighted_graph = env.maze.convert_to_weighted_graph(None, use_weighted_graph);
        Ok(env)
    }

    #[pyfunction(
        signature = (direction, run),
        text_signature = "(direction, run)"
    )]
    fn create_action(direction: usize, run: usize) -> PyResult<Action> {
        Ok(Action { direction, run })
    }

    /// Formats the sum of two numbers as string.
    #[pyfunction(
        signature = (environment),
        text_signature = "(environment)")]
    fn make_maze_imperfect(environment: &mut Environment) -> PyResult<()> {
        let walls_to_break = environment.maze.break_random_walls(15);
        environment.maze.break_walls_for_path(walls_to_break);
        environment.weighted_graph = environment.maze.convert_to_weighted_graph(None, true);
        Ok(())
    }

    #[pyfunction(
        signature = (environment),
        text_signature = "(environment)")]
    fn get_score(environment: &mut Environment) -> PyResult<ReportCard> {
        Ok(environment.generate_report_card())
    }
    #[pyfunction(
        signature = (json_str),
        text_signature = "(json_str)")]
    fn report_card_from_json(json_str: String) -> PyResult<ReportCard> {
        Ok(ReportCard::from_json(&json_str).unwrap())
    }

    #[pyfunction(
        signature = (),
        text_signature = "()")]
    fn maze_generation_algorithms() -> PyResult<Vec<String>> {
        Ok(MazeType::iter().map(|maze| maze.to_string()).collect())
    }

    #[pymodule]
    fn maze_library(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(init_environment, m)?)?;
        m.add_function(wrap_pyfunction!(create_action, m)?)?;
        m.add_function(wrap_pyfunction!(make_maze_imperfect, m)?)?;
        m.add_function(wrap_pyfunction!(get_score, m)?)?;
        m.add_function(wrap_pyfunction!(maze_generation_algorithms, m)?)?;
        m.add_function(wrap_pyfunction!(report_card_from_json, m)?)?;
        m.add_class::<Direction>()?;
        m.add_class::<Environment>()?;
        m.add_class::<Action>()?;
        m.add_class::<ActionResult>()?;
        m.add_class::<ReportCard>()?;
        Ok(())
    }
}
