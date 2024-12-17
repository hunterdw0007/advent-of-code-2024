use std::{
    fs::{read_to_string, File},
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    println!("Day-15");

    let lines = read_to_string("./data.txt").unwrap();
    let lines_split = lines.split_once("\n\n").unwrap();

    let mut area: Vec<Vec<char>> = lines_split
        .0
        .split('\n')
        .map(|r| r.chars().collect())
        .collect();
    let mut dirs: Vec<Direction> = lines_split
        .1
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    //print_2d_vec(&area);
    //println!("{:?}", dirs);

    for dir in dirs {
        let pos: (usize, usize) = area
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, &c)| c == '@')
                    .map(|(j, _)| (i, j))
            })
            .unwrap();
        try_move(&mut area, dir, pos);
        //print_2d_vec(&area);
    }
    //print_2d_vec(&area);

    let gps_sum: usize = area
        .iter()
        .enumerate()
        .flat_map(|(row, vec)| {
            vec.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'O')
                .map(move |(col, _)| 100 * row + col)
        })
        .sum();

    println!("GPS Sum: {}", gps_sum);
}

fn print_2d_vec(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|r| {
        r.iter().for_each(|c| print!("{}", c));
        println!()
    });
}

fn try_move(area: &mut Vec<Vec<char>>, dir: Direction, pos: (usize, usize)) -> bool {
    // base case - can move into empty space
    if area[pos.0][pos.1] == '.' {
        return true;
    }

    let (next_row, next_col) = match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    };

    // check if we can move in the desired direction
    if area
        .get(next_row)
        .and_then(|row| row.get(next_col))
        .map_or(false, |&c| c != '#')
        && try_move(area, dir, (next_row, next_col))
    {
        // can move, so swap elements
        let temp = area[pos.0][pos.1];
        area[pos.0][pos.1] = area[next_row][next_col];
        area[next_row][next_col] = temp;
        return true;
    }

    false
}
