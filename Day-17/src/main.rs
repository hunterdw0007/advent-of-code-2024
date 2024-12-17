use itertools::Itertools;
use std::{fs::read_to_string, u64};

fn main() {
    println!("Day-17");

    let lines = read_to_string("./data.txt").unwrap();
    let lines_split = lines.split_once("\n\n").unwrap();
    let mut registers: (u64, u64, u64) = lines_split
        .0
        .split('\n')
        .map(|l| l.split_once(": ").unwrap().1.parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap();
    let ops: Vec<u64> = lines_split
        .1
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|o| o.parse::<u64>().unwrap())
        .collect();

    //println!("{:?}", ops);

    let out = program(&mut registers, &ops);

    println!("Output: {}", out.into_iter().join(","));

    //Part 2
    /*
    Each 3 bits in A produces an output to the program
    The program is 16 numbers which means we need 3*16 bits in the Initial A to get a valid answer
    This happens in reverse order i.e. the 3 LSB in A correspond to the last op code
     */
    // let mut replicate: u64 = 0;
    // for op in ops.clone().iter().rev() {
    //     replicate <<= 3;
    //     registers.0 = replicate;
    //     while program(&mut registers, &ops)[0] != *op {
    //         replicate += 1;
    //         registers.0 = replicate;
    //     }
    //     println!("{:o}", replicate);
    // }

    let mut replicate: u64 = 0;
    for (i, op) in ops.clone().iter().rev().enumerate() {
        replicate <<= 3;
        registers.0 = replicate;
        'search: loop {
            let result = program(&mut registers, &ops);
            // Check against last i+1 elements of ops
            let target = &ops[ops.len() - (i + 1)..];
            if target == &result[..target.len()] {
                break 'search;
            }
            replicate += 1;
            registers.0 = replicate;
        }
        println!("{:o}", replicate);
    }

    println!("Replicate: {}", replicate);

    assert_eq!(ops, program(&mut (replicate, 0, 0), &ops));
}

fn program(registers: &mut (u64, u64, u64), ops: &Vec<u64>) -> Vec<u64> {
    let mut out = vec![];
    let mut ptr = 0_usize;
    while ptr < ops.len() {
        //println!("Registers: {:?}", registers);
        let op = ops[ptr];
        //println!("Op: {}", op);
        ptr += 1;

        let combo = get_combo(ops[ptr], &registers);
        let literal = ops[ptr];

        //println!("Combo: {}, Literal: {}", combo, literal);

        match op {
            0 => {
                registers.0 = registers.0 / (2_u64.pow(combo as u32));
                ptr += 1;
                continue;
            }
            1 => {
                registers.1 = registers.1 ^ (literal as u64);
                ptr += 1;
                continue;
            }
            2 => {
                registers.1 = combo.rem_euclid(8);
                ptr += 1;
                continue;
            }
            3 => {
                if registers.0 == 0 {
                    ptr += 1;
                    continue;
                } else {
                    ptr = literal as usize;
                    continue;
                }
            }
            4 => {
                registers.1 = registers.1 ^ registers.2;
                ptr += 1;
                continue;
            }
            5 => {
                out.push(combo.rem_euclid(8));
                ptr += 1;
                continue;
            }
            6 => {
                registers.1 = registers.0 / (2_u64.pow(combo as u32));
                ptr += 1;
                continue;
            }
            7 => {
                registers.2 = registers.0 / (2_u64.pow(combo as u32));
                ptr += 1;
                continue;
            }
            _ => panic!(),
        }
    }
    println!("{:?}", out);
    out
}

fn get_combo(op: u64, registers: &(u64, u64, u64)) -> u64 {
    match op {
        0..=3 => return op as u64,
        4 => return registers.0,
        5 => return registers.1,
        6 => return registers.2,
        7.. => panic!(),
    }
}
