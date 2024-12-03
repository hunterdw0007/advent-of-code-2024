use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    println!("Day 1");

    let mut first_numbers: Vec<u32> = Vec::new();
    let mut second_numbers: Vec<u32> = Vec::new();
    let mut number_map: HashMap<u32, u32> = HashMap::new();

    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let (first, second) = line.split_once("   ").unwrap();

            insert_sort(&mut first_numbers, first.parse::<u32>().unwrap());
            insert_sort(&mut second_numbers, second.parse::<u32>().unwrap());
            number_counts(&mut number_map, second.parse::<u32>().unwrap())
        }
    }

    let total_diff = compare_lists(&first_numbers, &second_numbers);

    println!("Total Difference: {}", total_diff);

    let similarity_score = compare_map(&first_numbers, &number_map);

    println!("Similarity Score: {}", similarity_score);
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

fn insert_sort(list: &mut Vec<u32>, num: u32) {
    list.push(num);
    let mut i = list.len() - 1;
    while i > 0 && list[i - 1] > list[i] {
        list.swap(i - 1, i);
        i -= 1;
    }
}

fn compare_lists(first: &Vec<u32>, second: &Vec<u32>) -> u32 {
    let mut total_diff = 0u32;
    for i in 0 .. first.len() {
        total_diff += first[i].abs_diff(second[i]);
    }
    total_diff
}

fn number_counts(map: &mut HashMap<u32, u32>, num: u32) {
    map.entry(num).and_modify(|i| {*i += 1}).or_insert(1u32);
}

fn compare_map(first: &Vec<u32>, map: &HashMap<u32, u32>) -> u32 {
    let mut similarity_score = 0u32;
    for elem in first.iter() {
        similarity_score += elem * map.get(elem).unwrap_or(&0);
    }
    similarity_score
}