import simulation

north = simulation.Direction.North
south = simulation.Direction.South
east = simulation.Direction.East
west = simulation.Direction.West

def dfs_solve(maze):
    step = 0
    stack = []
    available_paths = maze.available_paths()
    for direction in available_paths:
        stack.append((maze.current_location, step, direction))
    visited_set = {}
    path = []
    end = maze.end
    while len(stack) > 0:

        current = stack.pop()
        if (current[0], current[2]) in visited_set:
            continue
        if len(path) > 0 :         
            temp_current = path[len(path)- 1]
            while temp_current[1] > current[1]: 
                print("RUNNING")
                temp_current = path.pop()
                maze.move_from_current(temp_current[2].opposite_direction())
                
        print("current" , current)
        print("location", maze.current_location)
        print("location", maze.current_location == current[0])
        print("path", path)
        visited_set[((current[0], current[2]))] = 0
        step = current[1] + 1
        path.append(current)
        maze.move_from_current(current[2])
        if maze.current_location == end:
            return path
        available_paths = maze.available_paths()
        available_paths.remove(current[2].opposite_direction())
        for direction in available_paths:
            stack.append((maze.current_location, step, direction))

        

        
         
    

maze = simulation.init_maze_python(5,5)
print(maze.steps) 
simulation.create_wilsons_maze(maze)
print(dfs_solve(maze) )
print("DONE SOLVE")
print(maze.steps) 
print(maze.end) 
