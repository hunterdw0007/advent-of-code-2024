use std::fs::read_to_string;

fn main() {
    println!("Day 22");

    let lines = read_to_string("./data.txt").unwrap();
    let mut secrets: Vec<u64> = lines.split("\n").map(|s| s.parse().unwrap()).collect();
    //let mut secrets: Vec<u64> = vec![1, 10, 100, 2024];

    for i in 0..2000 {
        secrets.iter_mut().for_each(|s| evolve(s));
    }

    println!("Secrets: {:?}", secrets);
    println!("Sum: {}", secrets.iter().sum::<u64>());
}

fn evolve(secret: &mut u64) {
    // Step 1
    let mut mix = *secret << 6;
    *secret ^= mix;
    *secret = *secret % 16777216;

    //Step 2
    mix = *secret >> 5;
    *secret ^= mix;
    *secret = *secret % 16777216;

    //Step 3
    mix = *secret << 11;
    *secret ^= mix;
    *secret = *secret % 16777216;
}
