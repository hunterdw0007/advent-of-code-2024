use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
struct Calibration {
    total: u64,
    numbers: Vec<u64>,
}

fn main() {
    println!("Day 6");

    let mut calibrations: Vec<Calibration> = Vec::new();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let tot_nums: Vec<&str> = line.split(':').collect();

            let total: u64 = tot_nums[0].parse().unwrap();

            let nums: Vec<u64> = tot_nums[1]
                .trim()
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();

            calibrations.push(Calibration {
                total: total,
                numbers: nums,
            });
        }
    }

    //println!("{:?}", calibrations);
    let add = |a: u64, b: u64| -> u64 { a + b };
    let mul = |a: u64, b: u64| -> u64 { a * b };
    let ops = vec![add, mul];

    let total_p1 = solve(calibrations, ops);
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

fn solve<T>(calibrations: Vec<Calibration>, operators: Vec<T>) -> u64 where T: Fn(u64, u64) -> u64 {
    todo!()
}