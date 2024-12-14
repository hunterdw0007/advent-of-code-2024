use core::num;
use std::{
    fs::File,
    i32,
    io::{self, BufRead},
    path::Path,
};

use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: (i16, i16), // position can never be higher than (100, 102)
    velocity: (i16, i16),
}

fn main() {
    println!("Day 14");

    let mut robots: Vec<Robot> = vec![];
    let re: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let nums = re.captures(&line).unwrap();
            robots.push(Robot {
                position: (nums[1].parse().unwrap(), nums[2].parse().unwrap()),
                velocity: (nums[3].parse().unwrap(), nums[4].parse().unwrap()),
            });
        }
    }

    //println!("{:?}", robots);
    let mut seconds = 0;
    let x_size = 101;
    let y_size = 103;

    while seconds < x_size * y_size {
        for robot in robots.iter_mut() {
            move_robot(robot);
        }
        seconds += 1;
        if safety_factor(&robots, &seconds) {
            break;
        }
    }
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

fn move_robot(robot: &mut Robot) {
    let x_size = 101;
    let y_size = 103;
    // movement is pos + vel % size to loop
    let new_x = ((robot.position.0 + robot.velocity.0) + x_size) % x_size;
    let new_y = ((robot.position.1 + robot.velocity.1) + y_size) % y_size;
    robot.position = (new_x, new_y);
}

fn safety_factor(robots: &Vec<Robot>, seconds: &u32) -> bool {
    let x_size = 101;
    let y_size = 103;
    let x_mid = x_size / 2; // Will be 50
    let y_mid = y_size / 2; // Will be 51
    let mut factors = (0, 0, 0, 0);

    for robot in robots {
        match robot.position {
            (x, y) if x < x_mid && y < y_mid => factors.0 += 1,
            (x, y) if x < x_mid && y > y_mid => factors.1 += 1,
            (x, y) if x > x_mid && y < y_mid => factors.2 += 1,
            (x, y) if x > x_mid && y > y_mid => factors.3 += 1,
            _ => (),
        }
    }

    let factor = factors.0 * factors.1 * factors.2 * factors.3;

    if *seconds == 100 {
        println!("Seconds: {}", seconds);
        println!("Safety Factor: {}", factor);
        return false;
    }
    if no_collisions(robots) {
        println!("{}", visualize_robots(robots));
        println!("Seconds: {}", seconds);
        println!("Safety Factor: {}", factor);
        return true;
    }
    return false;
}

fn visualize_robots(robots: &Vec<Robot>) -> String {
    let width = 101;
    let height = 103;

    // Create a 2D vector to store counts, initialized to 0
    let mut counts = vec![vec![0u8; width]; height];

    // Count robots at each position
    for robot in robots {
        let (x, y) = robot.position;
        if x >= 0 && x < width as i16 && y >= 0 && y < height as i16 {
            counts[y as usize][x as usize] += 1;
        }
    }

    // Build the visualization string
    let mut result = String::new();

    for y in 0..height {
        for x in 0..width {
            if counts[y][x] == 0 {
                result.push(' ');
            } else {
                // Convert count to char, but cap at 9 for display
                let count = std::cmp::min(counts[y][x], 9);
                result.push(char::from_digit(count as u32, 10).unwrap());
            }
        }
        result.push('\n');
    }

    result
}

fn no_collisions(robots: &Vec<Robot>) -> bool {
    let mut seen_positions = std::collections::HashSet::new();

    for robot in robots {
        // Try to insert the position. insert() returns false if the element was already present
        if !seen_positions.insert(robot.position) {
            return false; // Found a duplicate position
        }
    }

    true // No duplicates found
}
