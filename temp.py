import simulation

north = simulation.Direction.North
south = simulation.Direction.South
east = simulation.Direction.East
west = simulation.Direction.West

def dfs_solve(environment):
    step = 1
    stack = []
    available_paths = environment.available_paths()
    print(available_paths)
    for direction in available_paths:
        stack.append((environment.current_location, step, direction))
    visited_set = {}
    path = []
    end = environment.maze.end
    while len(stack) > 0:

        current = stack.pop()
        if (current[0], current[2]) in visited_set:
            continue
        if len(path) > 0 :         
            temp_current = path[-1]
            while temp_current[1] >= current[1] and len(path) > 0: 
                environment.move_from_current(temp_current[2].opposite_direction())
                path.pop()
                temp_current = path[-1]
                
                
        
        visited_set[((current[0], current[2]))] = 0
        step = current[1] + 1
        path.append(current)
        environment.move_from_current(current[2])
        if environment.current_location == end:
            return path
        available_paths = environment.available_paths()
        available_paths.remove(current[2].opposite_direction())
        for direction in available_paths:
            stack.append((environment.current_location, step, direction))
   
         
    

environment = simulation.init_environment_python(15,15)
simulation.create_kruzkals_maze(environment)
print(dfs_solve(environment) )
print("DONE SOLVE")
with open('solve1.json', 'w') as file:
    file.write(environment.to_json())
