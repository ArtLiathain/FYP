import simulation

north = simulation.Direction.North
south = simulation.Direction.South
east = simulation.Direction.East
west = simulation.Direction.West

def dfs_solve(environment):
    step = 1
    stack = []
    available_paths = environment.reset()
    print(available_paths)
    for direction in available_paths:
        stack.append((environment.current_location, step, direction))
    visited_set = {}
    path = []
    while len(stack) > 0:

        current = stack.pop()
        if (current[0], current[2]) in visited_set:
            continue
        if len(path) > 0 :         
            temp_current = path[-1]
            while temp_current[1] >= current[1] and len(path) > 0: 
                temp_current = path[-1]
                result= environment.take_action(simulation.create_action(temp_current[2].opposite_direction()))
                available_paths = result.available_paths
                reward = result.reward
                is_done = result.is_done
                is_truncated = result.is_truncated

                path.pop()
                
                
        
        visited_set[((current[0], current[2]))] = 0
        step = current[1] + 1
        path.append(current)
        result = environment.take_action(simulation.create_action(current[2]))
        available_paths = result.available_paths
        reward = result.reward
        is_done = result.is_done
        is_truncated = result.is_truncated
                
        print(reward, is_truncated)
        if is_done:
            return path
        available_paths.remove(current[2].opposite_direction())
        for direction in available_paths:
            stack.append((environment.current_location, step, direction))
   
         
    

environment = simulation.init_environment_python(5,5)
simulation.create_kruzkals_maze(environment)
print(environment.take_action(simulation.create_action(north)))
print(dfs_solve(environment) )
print("DONE SOLVE")
with open('../mazeLogs/solve1.json', 'w') as file:
    file.write(environment.to_json())
