use regex::{Match, Regex};
use std::{fs::File, io::Read, path::Path};

fn main() {
    println!("Day 3");

    let mut mul_total: i32 = 0i32;
    let mul_regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    // Create a path to the desired file
    let path = Path::new("./data.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("file read successfully"),
    }
    println!("Is Ascii? {}", s.is_ascii());

    // Find and capture each instance of the regex
    for (full, [first, second]) in mul_regex.captures_iter(&s).map(|c| c.extract()) {
        //println!("Found match: {} with numbers {} and {}", full, first, second);
        mul_total += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }

    println!("Total: {}", mul_total);

    //println!("Enabled Total: {}", enabled_total(&s));

    println!("Improved Enabled Total: {}", improved(&s));
}

fn enabled_total(instructions: &String) -> i32 {
    let mul_regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_regex: Regex = Regex::new(r"do(n't)?\(\)").unwrap();
    let mut is_do = true; // start enabled
    let mut total = 0i32;
    let mut pos = 0;

    while pos < instructions.len() {
        let mul_match = mul_regex.find_at(&instructions, pos);
        let do_match = do_regex.find_at(&instructions, pos);
        //println!("Do: {} | Total: {} | Position: {}", is_do, total, pos);
        match (mul_match, do_match) {
            (Some(mm), Some(dm)) => {
                if mm.start() < dm.start() {
                    if is_do {
                        //println!("Found mul at {}", mm.start());
                        total += handle_mul(&mm);
                    }
                    pos = mm.end();
                } else {
                    //println!("Found do at {}", dm.start());
                    is_do = handle_do(&dm);
                    pos = dm.end();
                }
            }
            (Some(mm), None) => {
                if is_do {
                    //println!("Found mul at {}", mm.start());
                    total += handle_mul(&mm);
                }
                pos = mm.end();
            }
            (None, Some(dm)) => {
                //println!("Found do at {}", dm.start());
                is_do = handle_do(&dm);
                pos = dm.end();
            }
            (None, None) => break, // There are no more relevant instructions
        }
    }
    return total;
}

fn handle_mul(mat: &Match) -> i32 {
    let mul_regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let Some((full, [first, second])) = mul_regex.captures(mat.as_str()).map(|c| c.extract())
    else {
        return 0;
    };
    first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap()
}

fn handle_do(mat: &Match) -> bool {
    let do_regex: Regex = Regex::new(r"do(n't)?\(\)").unwrap();
    if let Some(caps) = do_regex.captures(mat.as_str()) {
        let dont = caps.get(1).map(|m| m.as_str());

        match dont {
            Some(_) => false, // has "n't"
            None => true,     // just "do()"
        }
    } else {
        false // no match at all
    }
}

fn improved(instructions: &String) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut pos = 0;
    let mut total = 0;
    let mut enabled = true;

    while pos < instructions.len() {
        let mat = match regex.find_at(&instructions, pos) {
            Some(m) => m,
            None => break
        };
        //println!("Position {} | Enabled {} | Match {} found at {}", pos, enabled, mat.as_str(), mat.start());

        if mat.as_str() == "do()" {
            enabled = true;
        }
        else if mat.as_str() == "don't()" {
            enabled = false;
        }
        // We can multiply and it's a mul instruction
        else if enabled && mat.as_str().starts_with("mul(") {
            let caps = regex.captures(&mat.as_str()).unwrap();
            //println!("Found match: {} with numbers {} and {}", full, first, second);
            total += caps.get(1).unwrap().as_str().parse::<i32>().unwrap() * caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        }
        // Move past the instruction
        pos = mat.start() + mat.as_str().len();
    }
    total
}
