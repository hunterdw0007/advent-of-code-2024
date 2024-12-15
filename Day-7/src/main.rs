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
    println!("Day 7");

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
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    for calibration in calibrations {
        total_p1 += solve(&calibration);
        total_p2 += solve_2(&calibration);
    }

    println!("Part 1 Total: {}", total_p1);
    println!("Part 2 Total: {}", total_p2);
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

fn solve(calibration: &Calibration) -> u64 {
    let nums = calibration.numbers.len(); // ex 6 numbers -> 5 operators

    let num_combinations = 1 << (nums - 1);  // 2^(n-1) combinations

    for i in 0..num_combinations {
        let mut result = calibration.numbers[0];
        
        // Process each operation from left to right
        for j in 1..nums {
            // Check if bit is 0 (+) or 1 (*)
            let operation_bit = (i >> (j - 1)) & 1;
            
            if operation_bit == 0 {
                result += calibration.numbers[j];
            } else {
                result *= calibration.numbers[j];
            }
        }
        if result == calibration.total {
            return calibration.total;
        }
    }
    return 0;
}

fn solve_2(calibration: &Calibration) -> u64 {
    let nums = calibration.numbers.len(); // ex 6 numbers -> 5 operators

    let num_combinations = 3_i64.pow((nums-1) as u32);  // 3^(n-1) combinations

    for i in 0..num_combinations {
        let mut result = calibration.numbers[0];
        let mut temp = i;
        // Process each operation from left to right
        for j in 1..nums {
            // Check if bit is 0 (+) or 1 (*)
            let operation = temp % 3;
            temp /= 3;
            
            match operation {
                0 => result += calibration.numbers[j],
                1 => result *= calibration.numbers[j],
                2 => {
                    let concat = format!("{}{}", result, calibration.numbers[j])
                        .parse::<u64>()
                        .unwrap();
                    result = concat;
                },
                _ => unreachable!()
            }
        }
        if result == calibration.total {
            return calibration.total;
        }
    }
    return 0;
}