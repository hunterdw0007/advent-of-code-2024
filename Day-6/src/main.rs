use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    println!("Day 6");

    let mut area: Vec<Vec<char>> = Vec::new();
    let mut direction: Direction = Direction::Up;
    let mut new_obstacles: Vec<(usize, usize)> = Vec::new();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            area.push(line.chars().collect());
        }
    }
    let mut position = find_start(&area);

    // mark the starting point as |
    area[position.0][position.1] = '|';

    // go until we reach an edge
    while !move_and_mark(&mut area, &mut position, &mut direction, &mut new_obstacles) {}

    print_2d_vec(&area);

    let mut visited = 0;
    for row in area {
        visited += row
            .iter()
            .filter(|c| **c == '|' || **c == '-' || **c == '+')
            .count();
    }

    println!("Visited Spaces: {}", visited);

    /*
    Part 2 Notes:
    Mark the positions like the example with +, -, and | ✅
    At each position, check if turning would get you to either a + or a - or | (depending on direction of travel)
    before you reached a # or the edge
    If you just turned ie a # is a rotation left of your position don't check since that would put you going backwards
    If you don't encounter a previous path by turning then move as before
    Add additional logic to check what character to put at each position
    Keep track of the possible loops using a separate list of (usize, usize) tuples

    It seems like it may have the possibility to create an instance where a theoretical turn then leads to a # 
    that causes a turn and a loop starts there but I am specifically excluding that scenario
    I need to make it so that it will turn itself when it reaches a #

    I implented the above scenario but I got stack overflow lol
    */

    println!("Possible New Obstacles: {:?}", new_obstacles);
    println!("Number of Possible New Obstacles: {}", new_obstacles.len());
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
    println!("{}", "Z".repeat(grid[0].len() + 2));
    for row in grid {
        print!("Z");
        for &c in row {
            if c == '.' {
                print!(" ");
            } else {
                print!("{}", c);
            }
        }
        print!("Z");
        println!();
    }
    println!("{}", "Z".repeat(grid[0].len() + 2));
}

fn find_start(area: &Vec<Vec<char>>) -> (usize, usize) {
    for (rn, row) in area.iter().enumerate() {
        for (cn, ch) in row.iter().enumerate() {
            if *ch == '^' {
                return (rn, cn);
            }
        }
    }
    unreachable!()
}

// returns false if we are not moving off the board and true if we are
fn move_and_mark(
    area: &mut Vec<Vec<char>>,
    pos: &mut (usize, usize),
    dir: &mut Direction,
    new_obstacles: &mut Vec<(usize, usize)>,
) -> bool {
    let edge = move_pos(dir, pos, area);
    mark_pos(area, pos, dir);

    match can_loop(dir, pos, area) {
        Some(x) => {
            new_obstacles.push(x);
            // println!("{:#?}", dir);
            // println!("{:?}", x);
            // print_2d_vec(&area);
        }
        None => (),
    }
    edge
}

fn mark_pos(area: &mut Vec<Vec<char>>, pos: &mut (usize, usize), dir: &mut Direction) {
    match dir {
        Direction::Up | Direction::Down => match area[pos.0][pos.1] {
            '.' => area[pos.0][pos.1] = '|',
            '-' => area[pos.0][pos.1] = '+',
            _ => return,
        },
        Direction::Right | Direction::Left => match area[pos.0][pos.1] {
            '.' => area[pos.0][pos.1] = '-',
            '|' => area[pos.0][pos.1] = '+',
            _ => return,
        },
    }
}

fn move_pos(dir: &mut Direction, pos: &mut (usize, usize), area: &mut Vec<Vec<char>>) -> bool {
    match dir {
        Direction::Up => {
            // not at the edge
            if pos.0 >= 1 {
                // can move so move
                if area[pos.0 - 1usize][pos.1] != '#' {
                    pos.0 -= 1;
                } else {
                    // can't move so rotate and move
                    *dir = Direction::Right;
                }
            } else {
                return true;
            }
        }
        Direction::Right => {
            // not at the edge
            if pos.1 < area[pos.0].len() - 1 {
                // can move so move
                if area[pos.0][pos.1 + 1] != '#' {
                    pos.1 += 1;
                } else {
                    // can't move so rotate and move
                    *dir = Direction::Down;
                }
            } else {
                return true;
            }
        }
        Direction::Down => {
            // not at the edge
            if pos.0 < area.len() - 1 {
                // can move so move
                if area[pos.0 + 1usize][pos.1] != '#' {
                    pos.0 += 1;
                } else {
                    // can't move so rotate and move
                    *dir = Direction::Left;
                }
            } else {
                return true;
            }
        }
        Direction::Left => {
            // not at the edge
            if pos.1 >= 1 {
                // can move so move
                if area[pos.0][pos.1 - 1] != '#' {
                    pos.1 -= 1;
                } else {
                    // can't move so rotate and move
                    *dir = Direction::Up;
                }
            } else {
                return true;
            }
        }
    }
    return false;
}

fn can_loop(
    dir: &mut Direction,
    pos: &mut (usize, usize),
    area: &mut Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    match dir {
        Direction::Up => {
            // just rotated
            if area[pos.0][pos.1 - 1] == '#' && area[pos.0][pos.1] == '+' {
                return None;
            }
            let mut i = 1;
            while i < area[0].len() - pos.1 {
                if area[pos.0][pos.1 + i] == '#' {
                    return can_loop(&mut Direction::Right, &mut(pos.0, pos.1 + i - 1), area);
                }
                if area[pos.0][pos.1 + i] == '+' && area[pos.0][pos.1 + 1 + i] == '#' {
                    return Some((pos.0 - 1, pos.1));
                }
                i += 1;
            }
        }
        Direction::Right => {
            // just rotated
            if area[pos.0 - 1][pos.1] == '#' && area[pos.0][pos.1] == '+' {
                return None;
            }
            let mut i = 1;
            while i < area.len() - pos.0 {
                if area[pos.0 + i][pos.1] == '#' {
                    return can_loop(&mut Direction::Down, &mut(pos.0 + i - 1, pos.1), area);
                }
                if area[pos.0 + i][pos.1] == '+' && area[pos.0 + 1 + i][pos.1] == '#'{
                    return Some((pos.0, pos.1 + 1));
                }
                i += 1;
            }
        }
        Direction::Down => {
            // just rotated
            if area[pos.0][pos.1 + 1] == '#' && area[pos.0][pos.1] == '+' {
                return None;
            }
            let mut i = 1;
            while i < pos.1 {
                if area[pos.0][pos.1 - i] == '#' {
                    return can_loop(&mut Direction::Left, &mut(pos.0, pos.1 - i + 1), area);
                }
                if area[pos.0][pos.1 - i] == '+' && area[pos.0][pos.1 - 1 - i] == '#' {
                    return Some((pos.0 + 1, pos.1));
                }
                i += 1;
            }
        }
        Direction::Left => {
            // just rotated
            if area[pos.0 + 1][pos.1] == '#' && area[pos.0][pos.1] == '+' {
                return None;
            }
            let mut i = 1;
            while i < pos.0 {
                if area[pos.0 - i][pos.1] == '#' {
                    return can_loop(&mut Direction::Up, &mut(pos.0 - i + 1, pos.1), area);
                }
                if area[pos.0 - i][pos.1] == '+' && area[pos.0 - 1 - i][pos.1] == '#'{
                    return Some((pos.0, pos.1 - 1));
                }
                i += 1;
            }
        }
    }
    return None;
}
