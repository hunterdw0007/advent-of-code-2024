use std::{fs::File, io::Read, path::Path};
use regex::Regex;

fn main() {
    println!("Day 3");

    let mut mul_total: i32 = 0i32;
    let regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

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
    for (full, [first, second]) in regex.captures_iter(&s).map(|c| c.extract()) {
        //println!("Found match: {} with numbers {} and {}", full, first, second);
        mul_total += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }

    println!("Total: {}", mul_total)

}