use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvConfig {
    pub maze_width: usize,
    pub maze_height: usize,
    pub python_config: PythonConfig,
}

impl EnvConfig {
    pub fn new(maze_width: usize, maze_height: usize, python_config: PythonConfig) -> EnvConfig {
        EnvConfig {
            maze_width,
            maze_height,
            python_config,
        }
    }

    pub fn new_rust_config(maze_width: usize, maze_height: usize) -> EnvConfig {
        EnvConfig {
            maze_width,
            maze_height,
            python_config: PythonConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PythonConfig {
    pub allowed_revisits: usize,
}
