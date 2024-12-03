use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Day 2");

    let mut safe_reports_count = 0u32;
    let mut safe_reports_count_dampened = 0u32;

    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            if check_report(&report) {
                safe_reports_count += 1;
            }
            if check_report_dampened(&report) {
                safe_reports_count_dampened += 1;
            }
        }
    }

    println!("Safe Reports: {}", safe_reports_count);
    println!("Safe Reports Dampened: {}", safe_reports_count_dampened);
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

fn check_report_dampened(report: &Vec<i32>) -> bool {
    let mut valid = false;

    for i in 0..report.len() {
        let mut modified_report: Vec<i32> = Vec::new();
        modified_report.extend_from_slice(&report[..i]);
        modified_report.extend_from_slice(&report[i+1..]);

        valid = check_report(&modified_report);

        if valid {
            break
        }
    }
    valid
}

fn check_report(report: &Vec<i32>) -> bool {
    let mut valid: bool = true;

    let increasing: i32 = report[1] - report[0];

    if increasing == 0 {
        return !valid;
    }
    for i in 1 .. report.len() {
        if increasing > 0 {
            valid = check_increasing(&report[i-1], &report[i]);
        }
        if increasing < 0 {
            valid = check_decreasing(&report[i-1], &report[i]);
        }
        if !valid {
            break
        }
    }

    valid
}

fn check_increasing(elem1: &i32, elem2: &i32) -> bool {
    if elem2 - elem1 < 1 || elem2 - elem1 > 3 {
        return false;
    }
    return true
}

fn check_decreasing(elem1: &i32, elem2: &i32) -> bool {
    if elem1 - elem2 < 1 || elem1 - elem2 > 3 {
        return false;
    }
    return true
}