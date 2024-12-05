use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};

fn main() {
    println!("Day 5");

    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if line.contains('|') {
                let bef_aft: Vec<u32> = line.as_str().split('|').map(|x| x.parse().unwrap()).collect();
                rules.push((bef_aft[0], bef_aft[1]));
            }
            else if line.contains(",") {
                let pages: Vec<u32> = line.as_str().split(",").map(|x| x.parse().unwrap()).collect();
                updates.push(pages);
            }
        }
    }

    //println!("Rules: {:?}", rules);
    //println!("Updates: {:?}", updates);

    let mut total = 0;
    let mut fixed = 0;

    for mut update in updates {
        let middle = update.len() / 2;
        if check_update(&rules, &update) {
            //println!("Middle: {} at {}", update[middle], middle);
            total += update[middle];
            //println!("Total: {}", total)
        } else {
            fixed += fix_update(&rules, &mut update);
        }
    }

    println!("Total Correctly: {}", total);
    println!("Total Fixed: {}", fixed);
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

fn check_rule(rule: &(u32, u32), pages: &Vec<u32>) -> bool {
    if !(pages.contains(&rule.0) && pages.contains(&rule.1)) {
        return true;
    }
    let before = pages.iter().position(|&x| x == rule.0).unwrap();
    let after = pages.iter().position(|&x| x == rule.1).unwrap();
    before < after
}

fn check_update(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> bool {
    let mut valid = true;
    for rule in rules {
        valid = check_rule(rule, update);
        //println!("Update: {:?} | Rule: {:?} | Valid: {}", update, rule, valid);
        if !valid { break; }
    }
    valid
}

fn fix_update(rules: &Vec<(u32, u32)>, update: &mut Vec<u32>) -> u32 {
    for rule in rules {
        follow_rule(&rule, update);
    }
    // This is foul and could result in an infinite loop (I think) but it worked :)
    if !check_update(rules, update) { 
        //println!("Something went wrong with update: {:?}", update);
        fix_update(rules, update);
    }
    update[update.len() / 2]
}

fn follow_rule(rule: &(u32, u32), pages: &mut Vec<u32>) {
    if !(pages.contains(&rule.0) && pages.contains(&rule.1)) {
        return
    }
    //println!("Update: {:?} | Rule: {:?}", pages, rule);
    let before = pages.iter().position(|&x| x == rule.0).unwrap();
    let after = pages.iter().position(|&x| x == rule.1).unwrap();
    if before > after {
        //println!("BEFORE Update: {:?} | Rule: {:?}", pages, rule);
        let moved = pages.remove(after);
        pages.insert(before, moved);
        //println!("AFTER Update: {:?} | Rule: {:?}", pages, rule);
    }
}