use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/*
Notes:

Store the input as a Vec<Vec<(char, bool)>> to keep track of the region and whether it's been visited

Use a breadth first search recursive algorithm to propagate throughout the entire region to determine
    Be smart and check if it orthogonally borders another letter and only recursively call the letters that are in my region ie if moving down changes the letter don't call that direction
    Also make sure I don't try to backtrack and make infinite recursion by moving back and forth from the same space ie if my direction is down don't check up
    Store the directions in an enum with a None direction for if I am checking a region for the first time
    Add up each time to determine how many spaces make up the region
    Determine perimeter by checking each space to see if it borders any different letters in all orthogonal directions and add those up

Once I get to the end of the list I can do the sum of the region list
*/
enum Directions {
    None,
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    println!("Day 12");

    let mut garden: Vec<Vec<(char, bool)>> = vec![];

    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            garden.push(line.chars().map(|c| (c, false)).collect());
        }
    }

    let mut total_price = 0;

    loop {
        let position = find_unexplored(&garden);

        if position.is_err() {
            break;
        }
        let mut area = 0;
        let mut perimeter = 0;
        total_price += explore_region(&mut garden, position.unwrap(), &mut area, &mut perimeter);
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

fn find_unexplored(garden: &Vec<Vec<(char, bool)>>) -> Result<(usize, usize), &'static str> {
    for (row_pos, row) in garden.iter().enumerate() {
        let col = row.iter().position(|&p| p.1 == false);
        match col {
            Some(col) => {
                return Ok((row_pos, col));
            }
            None => continue, // Not in this row so keep trying
        }
    }
    return Err("Done");
}

fn explore_region(
    garden: &mut Vec<Vec<(char, bool)>>,
    position: (usize, usize),
    area: &mut usize,
    perimeter: &mut usize,
) {
    // always increment area since we move to a new space even in the base case
    *area += 1;

    // check up
    // will never move up because we start from the top leftmost position in the region and move right/left before down

    //check right

    //check left

    //check down


    // base case
    // moving in any direction results in either another letter or an explored space
    return ();
}
