use std::{cmp, collections::HashMap, fs::read_to_string, i32};

fn main() {
    println!("Day-19");

    let lines = read_to_string("./data.txt").unwrap();
    let lines_split = lines.split_once("\n\n").unwrap();
    let towels: Vec<&str> = lines_split.0.split(", ").collect();
    let patterns: Vec<&str> = lines_split.1.split("\n").collect();

    //println!("{:?}", towels);
    //println!("{:?}", patterns);

    let mut possible = 0;
    let mut tot_match = 0;
    let mut cache: Vec<&str> = vec![];
    let mut failed_cache: Vec<&str> = vec![];
    let mut cache_map = HashMap::new();

    for pattern in patterns {
        println!("Pattern: {}", pattern);
        if try_match(pattern, &towels, &mut cache, &mut failed_cache) {
            possible += 1;
            println!("Pattern: {} is possible", pattern);
        }
        tot_match += count_patterns(pattern, &towels, &mut cache_map);
    }

    println!("Possible: {}", possible);
    println!("Total Possible: {}", tot_match);

}

// returns true if a group of towels can be used to make the pattern
fn try_match<'a>(pattern: &'a str, towels: &Vec<&str>, cache: &mut Vec<&'a str>, failed_cache: &mut Vec<&'a str>) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if cache.contains(&pattern) {
        return true;
    }
    if failed_cache.contains(&pattern) {
        return false;
    }

    let min = towels.iter().map(|s| s.len()).min().unwrap_or(0);
    let max = towels.iter().map(|s| s.len()).max().unwrap_or(0);
    
    // Only check lengths that could actually match towels
    for i in min..=cmp::min(pattern.len(), max) {
        let (prefix, remaining) = pattern.split_at(i);
        if towels.contains(&prefix) {
            if try_match(remaining, towels, cache, failed_cache) {
                cache.push(pattern);
                return true;
            }
        }
    }

    failed_cache.push(pattern);
    false
}

fn count_patterns<'a>(pattern: &'a str, towels: &Vec<&str>, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if pattern.is_empty() {
        return 1; // Empty pattern can be made in exactly one way
    }
    
    // Check if we've already computed this pattern
    if let Some(&count) = cache.get(pattern) {
        return count;
    }
    
    let min = towels.iter().map(|s| s.len()).min().unwrap_or(0);
    let max = towels.iter().map(|s| s.len()).max().unwrap_or(0);
    
    let mut total = 0;
    // Try each possible prefix that could match a towel
    for i in min..=cmp::min(pattern.len(), max) {
        let (prefix, remaining) = pattern.split_at(i);
        if towels.contains(&prefix) {
            total += count_patterns(remaining, towels, cache);
        }
    }
    
    cache.insert(pattern, total);
    total
}