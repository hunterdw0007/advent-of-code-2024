use std::{
    collections::HashMap, fs::File, io::{self, BufRead}, path::Path
};

fn main() {
    println!("Day 11");

    let mut stones: Vec<u64> = Vec::new();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let mut line_split: Vec<u64> = line.split(" ").map(|x| x.parse().unwrap()).collect();
            stones.append(&mut line_split);
        }
    }

    println!("{:?}", stones);

    let mut total_len = 0;

    for stone in stones {
        //process_stones(&mut stones);
        //process_stones_reverse(&mut stones);
        //total_len += process_stones_recursive(stone, 75, 1);
        total_len += calculate_final_length(stone, 75);
    }

    // println!("{:?}", stones);
    println!("Length: {}", total_len);

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

fn process_stones(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let stone = stones[i];
        
        if stone == 0 {
            stones[i] = 1;
            i += 1;
        }
        else if has_even_digits(stone) {
            let split = split_number(stone);
            stones[i] = split.0;
            stones.insert(i + 1, split.1);
            i += 2;
        }
        else {
            stones[i] = stone * 2024;
            i += 1;
        }
    }
}

fn process_stones_reverse(stones: &mut Vec<u64>) {
    // First count total needed space
    let mut additional_space = 0;
    for &stone in stones.iter() {
        if stone != 0 && has_even_digits(stone) {
            additional_space += 1;
        }
    }
    
    // Pre-allocate space
    let original_len = stones.len();
    stones.resize(original_len + additional_space, 0);
    
    // Process from back to front
    let mut write_idx = stones.len() - 1;
    let mut read_idx = original_len - 1;
    
    while read_idx < stones.len() {  // Using < because read_idx is usize
        let stone = stones[read_idx];
        
        if stone == 0 {
            if write_idx >= 0 {
                stones[write_idx] = 1;
                write_idx = write_idx.saturating_sub(1);
            }
        } else if has_even_digits(stone) {
            let split = split_number(stone);
            if write_idx >= 1 {  // Make sure we have space for both numbers
                stones[write_idx] = split.1;
                stones[write_idx - 1] = split.0;
                write_idx = write_idx.saturating_sub(2);
            }
        } else {
            if write_idx >= 0 {
                stones[write_idx] = stone * 2024;
                write_idx = write_idx.saturating_sub(1);
            }
        }
        
        if read_idx == 0 { break; }
        read_idx -= 1;
    }
}

fn process_stones_recursive(stone: u64, steps_rem: usize, length: usize) -> usize {
    // base case
    if steps_rem == 0 {
        return length;
    }
    if stone == 0 {
       process_stones_recursive(1, steps_rem.saturating_sub(1), length)
    }
    else if has_even_digits(stone) {
        let split = split_number(stone);
        let mut out = 0;
        out += process_stones_recursive(split.0, steps_rem.saturating_sub(1), length);
        out += process_stones_recursive(split.1, steps_rem.saturating_sub(1), length);
        return out;
    }
    else {
        process_stones_recursive(stone * 2024, steps_rem.saturating_sub(1), length)
    }
}

fn process_stones_memoized(stone: u64, steps_rem: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    // Check cache first
    if let Some(&result) = cache.get(&(stone, steps_rem)) {
        return result;
    }
    
    // Base case
    if steps_rem == 0 {
        return 1;  // Each stone contributes 1 to length
    }

    let result = if stone == 0 {
        process_stones_memoized(1, steps_rem - 1, cache)
    } else if has_even_digits(stone) {
        let split = split_number(stone);
        process_stones_memoized(split.0, steps_rem - 1, cache) + 
        process_stones_memoized(split.1, steps_rem - 1, cache)
    } else {
        process_stones_memoized(stone * 2024, steps_rem - 1, cache)
    };

    // Cache the result
    cache.insert((stone, steps_rem), result);
    result
}

// Usage:
fn calculate_final_length(initial_stone: u64, steps: usize) -> usize {
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();
    process_stones_memoized(initial_stone, steps, &mut cache)
}

fn has_even_digits(mut num: u64) -> bool {
    let mut count = 1;
    while num >= 10 {
        num /= 10;
        count += 1;
    }
    count % 2 == 0
}

fn split_number(num: u64) -> (u64, u64) {
    let num_digits = (num as f64).log10().floor() as u64 + 1;
    let divisor = 10_u64.pow((num_digits as u32) / 2);
    (num / divisor, num % divisor)
}
