use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet}, vec,
};

use maze_library::environment::environment::{Coordinate, Environment};

pub fn dijksta_solve(env: &mut Environment) {
    let mut main_stack = vec![env.maze.get_starting_point()]; // Stack for DFS
    let mut path_map: HashMap<Coordinate, (usize, Coordinate)> = HashMap::new();
    let mut visited = HashSet::new(); // Track visited cells

    let weighted_graph = env.maze.convert_to_weighted_graph();
    while let Some(current) = main_stack.pop() {
        visited.insert(current);
        let mut sub_stack: Vec<(Coordinate, usize)> = weighted_graph
            .get(&current)
            .unwrap()
            .iter()
            .map(|(k, &v)| (env.maze.move_from(k, &current, Some(v)).unwrap(), v))
            .filter(|(coordinate, _)| !visited.contains(coordinate))
            .collect();
        sub_stack.sort_by_key(|&(_, v)| Reverse(v));
        main_stack.extend(sub_stack.iter().map(|(key, _)| key));
        while let Some((check_node, steps)) = sub_stack.pop() {
            let (current_steps, _) = *path_map.get(&current).unwrap_or(&(10000000, (0,0)));
            let (check_node_steps, _) =  match path_map.get(&check_node){
                Some(value) => *value,
                None =>  {path_map.insert(check_node, (current_steps + steps, current)); continue;}
            };
            if check_node_steps > current_steps + steps {
                path_map.insert(check_node, (current_steps + steps, current));
            }
        }
    }
    let mut head = env.maze.end;
    let (final_steps,_) = path_map.get(&env.maze.end).unwrap();
    env.steps = *final_steps;
    let mut path_followed= vec![];
    while head != env.maze.start{
        path_followed.push(head);
        let (_, temp) = path_map.get(&head).unwrap();
        head = *temp;
    }
    path_followed.push(env.maze.start);
    path_followed.reverse();
    env.path_followed = path_followed;
}
