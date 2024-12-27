use std::{collections::VecDeque, fs::{read_to_string, File}, io::{self, BufRead}, path::Path};

fn main() {
    println!("Day-20");

    let mut maze: Vec<Vec<char>> = vec![];

    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            maze.push(line.chars().collect());
        }
    }
    print_2d_vec(&maze);

    let mut original_len = 0;
    if let Some(path) = find_path(&maze) {
        println!("Path found with length: {}", path.len());
        original_len = path.len();
    } else {
        println!("No path found!");
    }

    let mut total = 0;
    for row in 1..maze.len()-1 {
        for col in 1..maze[0].len()-1 {
            if maze[row][col] == '#' {
                let mut new_maze = maze.clone();
                new_maze[row][col] = '.';
                if find_path(&new_maze).unwrap().len() <= original_len-100 {
                    total += 1;
                    println!("Cheat path found at position: ({},{})", row, col);
                }
            }
        }
    }

    println!("Total: {}", total);

}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_2d_vec(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|r| {
        r.iter().for_each(|c| print!("{}", c));
        println!()
    });
}

// Position in the maze
#[derive(Clone, Debug, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

fn find_path(maze: &Vec<Vec<char>>) -> Option<Vec<Pos>> {
    let rows = maze.len();
    let cols = maze[0].len();
    
    // Find start position
    let mut start = None;
    for i in 0..rows {
        for j in 0..cols {
            if maze[i][j] == 'S' {
                start = Some(Pos { row: i, col: j });
                break;
            }
        }
    }
    
    let start = start?;
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; cols]; rows];
    let mut parent: Vec<Vec<Option<Pos>>> = vec![vec![None; cols]; rows];
    
    queue.push_back(start.clone());
    visited[start.row][start.col] = true;
    
    // Possible moves: up, right, down, left
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    while let Some(current) = queue.pop_front() {
        // Check if we reached the end
        if maze[current.row][current.col] == 'E' {
            // Reconstruct path
            let mut path = Vec::new();
            let mut pos = current;
            path.push(pos.clone());
            
            while let Some(prev) = &parent[pos.row][pos.col] {
                path.push(prev.clone());
                pos = prev.clone();
            }
            
            path.reverse();
            return Some(path);
        }
        
        // Try all directions
        for (dx, dy) in directions.iter() {
            let new_row = current.row as i32 + dx;
            let new_col = current.col as i32 + dy;
            
            // Check bounds
            if new_row >= 0 && new_row < rows as i32 && 
               new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                // Check if we can move here (not wall and not visited)
                if !visited[new_row][new_col] && maze[new_row][new_col] != '#' {
                    visited[new_row][new_col] = true;
                    parent[new_row][new_col] = Some(current.clone());
                    queue.push_back(Pos { row: new_row, col: new_col });
                }
            }
        }
    }
    
    None // No path found
}