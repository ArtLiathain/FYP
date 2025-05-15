# Testing Environmental Bias
This project is in two secions
- A Rust Maze envirornment
- A Python PPO and Double DQN implementation to run the environment

The goal of this project was to create a testing bed to test rl agents to see if they adapt and learn the structural biases in maze generation algorithms.

This was created for my FYP.

## Using the project
1. Clone the repo down
2. Download the latest version of rust 
3. Download python 3.10 or newer
4. Setup a virtual environment 
```python3 -m venv .```
5. Pip install requirements.txt ```pip install -r requirements.txt```
6. Run ```cd RustMazeEnvironment```
7. Run ```maturin develop --manifest-path maze_library/Cargo.toml --features python```
8. To use the python code, navigate to any of the python notebooks and click run all.
9. For visualisation or working directly on the rust code there is a CLI which can be accessed using ````cargo run -- help````
