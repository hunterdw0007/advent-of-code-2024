use std::{fs::File, io::Read, path::Path};

fn main() {
    println!("Day 3");

    // Create a path to the desired file
    let path = Path::new("./data.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut input = String::new();
    let grid: Vec<Vec<char>> = match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => input.lines().map(|line| line.chars().collect()).collect(),
    };

    
    //padding to prevent index out of bounds
    let mut padded: Vec<Vec<char>> = Vec::new();
    
    // Add top row of #
    padded.push(vec!['#'; grid[0].len() + 2]);
    
    // Add # to start/end of each row
    for row in &grid[..] {
        let mut new_row = vec!['#'];
        new_row.extend(row);
        new_row.push('#');
        padded.push(new_row);
    }
    
    // Add bottom row of #
    padded.push(vec!['#'; grid[0].len() + 2]);
    
    let xmas_total = xmas_total(&padded);

    println!("Total XMAS: {}", xmas_total);

    let x_mas_total = x_mas_total(&padded);
    println!("Total X-MAS: {}", x_mas_total);
}

fn x_mas_total(padded: &Vec<Vec<char>>) -> i32 {
    let mut x_mas_total = 0;
    for (lnum, line) in padded.iter().enumerate() {
        for (cnum, char) in line.iter().enumerate() {
            if padded[lnum][cnum] == 'A'
            {
                // M  
                //  A 
                //   S
                if padded[lnum - 1][cnum - 1] == 'M' && padded[lnum + 1][cnum + 1] == 'S' {
                    // M M
                    //  A 
                    // S S
                    if padded[lnum - 1][cnum + 1] == 'M' && padded[lnum + 1][cnum - 1] == 'S' {
                        x_mas_total += 1;
                    }
                    // M S
                    //  A 
                    // M S
                    if padded[lnum - 1][cnum + 1] == 'S' && padded[lnum + 1][cnum - 1] == 'M' {
                        x_mas_total += 1;
                    }

                }
                // S  
                //  A 
                //   M
                if padded[lnum - 1][cnum - 1] == 'S' && padded[lnum + 1][cnum + 1] == 'M' {
                    // M M
                    //  A 
                    // S S
                    if padded[lnum - 1][cnum + 1] == 'M' && padded[lnum + 1][cnum - 1] == 'S' {
                        x_mas_total += 1;
                    }
                    // M S
                    //  A 
                    // M S
                    if padded[lnum - 1][cnum + 1] == 'S' && padded[lnum + 1][cnum - 1] == 'M' {
                        x_mas_total += 1;
                    }

                }
            }
        
        }
    }
    x_mas_total
}

fn xmas_total(padded: &Vec<Vec<char>>) -> i32 {
    let mut xmas_total = 0;
    for (lnum, line) in padded.iter().enumerate() {
        for (cnum, char) in line.iter().enumerate() {
            // right
            if padded[lnum][cnum] == 'X'
                && padded[lnum][cnum + 1] == 'M'
                && padded[lnum][cnum + 2] == 'A'
                && padded[lnum][cnum + 3] == 'S'
            {
                xmas_total += 1;
            }
            // left
            if padded[lnum][cnum] == 'X'
                && padded[lnum][cnum - 1] == 'M'
                && padded[lnum][cnum - 2] == 'A'
                && padded[lnum][cnum - 3] == 'S'
            {
                xmas_total += 1;
            }
            // down
            if padded[lnum][cnum] == 'X'
                && padded[lnum + 1][cnum] == 'M'
                && padded[lnum + 2][cnum] == 'A'
                && padded[lnum + 3][cnum] == 'S'
            {
                xmas_total += 1;
            }
            // up
            if padded[lnum][cnum] == 'X'
                && padded[lnum - 1][cnum] == 'M'
                && padded[lnum - 2][cnum] == 'A'
                && padded[lnum - 3][cnum] == 'S'
            {
                xmas_total += 1;
            }
            // diagonal right down
            if padded[lnum][cnum] == 'X'
                && padded[lnum + 1][cnum + 1] == 'M'
                && padded[lnum + 2][cnum + 2] == 'A'
                && padded[lnum + 3][cnum + 3] == 'S'
            {
                xmas_total += 1;
            }
             // diagonal left down
             if padded[lnum][cnum] == 'X'
             && padded[lnum + 1][cnum - 1] == 'M'
             && padded[lnum + 2][cnum - 2] == 'A'
             && padded[lnum + 3][cnum - 3] == 'S'
            {
                xmas_total += 1;
            }
            // diagonal right up
            if padded[lnum][cnum] == 'X'
                && padded[lnum - 1][cnum + 1] == 'M'
                && padded[lnum - 2][cnum + 2] == 'A'
                && padded[lnum - 3][cnum + 3] == 'S'
            {
                xmas_total += 1;
            }
            // diagonal left up
            if padded[lnum][cnum] == 'X'
                && padded[lnum - 1][cnum - 1] == 'M'
                && padded[lnum - 2][cnum - 2] == 'A'
                && padded[lnum - 3][cnum - 3] == 'S'
            {
                xmas_total += 1;
            }
        }
    }
    xmas_total
    }
