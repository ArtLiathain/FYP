import simulation

north = simulation.Direction.North
south = simulation.Direction.South
east = simulation.Direction.East
west = simulation.Direction.West

def dfs_solve(maze):
    stack = [maze.start]
    visited_set = {}
    path = []
    end = maze.end
    while len(stack) != 0:
        current = stack.pop()
        if current in visited_set:
            continue
        
         
    

maze = simulation.init_maze_python(5,5)
print(maze.steps) 
simulation.create_wilsons_maze(maze)
print(maze.available_walls()) 
print(maze.move_from_current(north)) 
print(maze.move_from_current(north)) 
print(maze.move_from_current(east)) 
print(maze.move_from_current(east)) 
print(maze.steps) 
print(maze.end) 
