use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    println!("Day-18");

    let lines = read_to_string("./data.txt").unwrap();
    let lines_split: Vec<&str> = lines.split("\n").collect();
    let mem_regex: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut mem: Vec<(usize, usize)> = vec![];
    for line in &lines_split[0..lines_split.len() - 1] {
        let caps = mem_regex.captures(line).unwrap();
        mem.push((caps[1].parse().unwrap(), caps[2].parse().unwrap()));
    }

    let mut area: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    for (col, row) in &mem[0..1024] {
        area[*row][*col] = '#';
    }
    //print_2d_vec(&area);

    match find_path(&area, (0, 0), (70, 70)) {
        Some(path) => {
            println!("Path found! Length: {}", path.len() - 1);
            print_path(&area, &path);
        }
        None => println!("No path found!"),
    }

    for (col, row) in &mem[1024..] {
        area[*row][*col] = '#';
        match find_path(&area, (0, 0), (70, 70)) {
            Some(_) => (),
            None => {
                println!("No path found! Blocker {},{}", col, row);
                break;
            }
        }
    }
}

fn print_2d_vec(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|r| {
        r.iter().for_each(|c| print!("{}", c));
        println!()
    });
}

fn get_neighbors(pos: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (x, y) = pos;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if new_x < grid[0].len() && new_y < grid.len() && grid[new_y][new_x] == '.' {
                neighbors.push((new_x, new_y));
            }
        }
    }
    neighbors
}

fn find_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = Vec::new();
            let mut current = current;
            path.push(current);
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();
            return Some(path);
        }

        for next in get_neighbors(current, grid) {
            if !visited.contains(&next) {
                visited.insert(next);
                came_from.insert(next, current);
                queue.push_back(next);
            }
        }
    }
    None
}

fn print_path(grid: &Vec<Vec<char>>, path: &[(usize, usize)]) {
    let mut display = grid.clone();

    for &(x, y) in path {
        display[y][x] = 'o';
    }

    if let Some(&(x, y)) = path.first() {
        display[y][x] = 'S';
    }
    if let Some(&(x, y)) = path.last() {
        display[y][x] = 'E';
    }

    for row in &display {
        println!("{}", row.iter().collect::<String>());
    }
}
