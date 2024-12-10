use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    println!("Day 6");

    let mut topo: Vec<Vec<u32>> = Vec::new();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            topo.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }
    }

    print_2d_vec(&topo);

    let zeroes: Vec<(usize, usize)> = find_zeroes(&topo);
    let mut unique_paths = 0;
    let mut distinct_paths = 0;

    for zero in zeroes {
        print!("Zero at: {:?} with paths to -> ", zero);
        let mut nines: Vec<(usize, usize)> = vec![];
        search(&topo, zero, &mut nines);
        nines.sort();
        distinct_paths += nines.len();
        nines.dedup();
        print!("{:?}", nines);
        unique_paths += nines.len();
        println!();
    }

    println!("Unique Paths: {}", unique_paths);
    println!("Distinct Paths: {}", distinct_paths);
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

fn print_2d_vec(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for &c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn find_zeroes(topo: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];
    for row in 0..topo.len() {
        for col in 0..topo[row].len() {
            if topo[row][col] == 0u32 {
                output.push((row, col));
            }
        }
    }
    output
}

fn search(topo: &Vec<Vec<u32>>, loc: (usize, usize), nines: &mut Vec<(usize, usize)>) {
    let mut total = 0;
    if topo[loc.0][loc.1] == 9 {
        nines.push(loc);
        return ();
    }
    // up
    if loc.0 > 0 {
        let new_loc = (loc.0 - 1, loc.1);
        if topo[new_loc.0][new_loc.1] > topo[loc.0][loc.1]
            && topo[new_loc.0][new_loc.1] - topo[loc.0][loc.1] == 1
        {
            search(topo, new_loc, nines);
        }
    }
    // left
    if loc.1 > 0 {
        let new_loc = (loc.0, loc.1 - 1);
        if topo[new_loc.0][new_loc.1] > topo[loc.0][loc.1]
            && topo[new_loc.0][new_loc.1] - topo[loc.0][loc.1] == 1
        {
            search(topo, new_loc, nines);
        }
    }
    // right
    if loc.1 < topo[loc.0].len() - 1 {
        let new_loc = (loc.0, loc.1 + 1);
        if topo[new_loc.0][new_loc.1] > topo[loc.0][loc.1]
            && topo[new_loc.0][new_loc.1] - topo[loc.0][loc.1] == 1
        {
            search(topo, new_loc, nines);
        }
    }
    // down
    if loc.0 < topo.len() - 1 {
        let new_loc = (loc.0 + 1, loc.1);
        if topo[new_loc.0][new_loc.1] > topo[loc.0][loc.1]
            && topo[new_loc.0][new_loc.1] - topo[loc.0][loc.1] == 1
        {
            search(topo, new_loc, nines);
        }
    }
}
