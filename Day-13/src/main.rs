use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct ClawMachine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    p_x: i64,
    p_y: i64,
}

fn main() {
    println!("Day 12");

    let mut claw_machines: Vec<ClawMachine> = vec![];

    let content = read_to_string("./data.txt").expect("Could not read file");

    // Split content into sections by double newlines
    let sections = content.split("\n\n");

    for section in sections {
        let re = Regex::new(r"^.*X.(\d+).*Y.(\d+)$").unwrap();
        // Split each section into lines
        let lines: Vec<&str> = section.lines().collect();

        // Make sure we have exactly 3 lines
        if lines.len() == 3 {
            let cap_a = re.captures(&lines[0]).unwrap();
            let cap_b = re.captures(&lines[1]).unwrap();
            let cap_p = re.captures(&lines[2]).unwrap();

            claw_machines.push(ClawMachine {
                a_x: cap_a.get(1).unwrap().as_str().parse().unwrap(),
                a_y: cap_a.get(2).unwrap().as_str().parse().unwrap(),
                b_x: cap_b.get(1).unwrap().as_str().parse().unwrap(),
                b_y: cap_b.get(2).unwrap().as_str().parse().unwrap(),
                p_x: cap_p.get(1).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000_i64,
                p_y: cap_p.get(2).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000_i64,
            });
        }
    }

    let mut tokens = 0;

    for claw_machine in claw_machines {
        tokens += play_game(&claw_machine);
        println!("{}", tokens);
    }
}

fn play_game(claw_machine: &ClawMachine) -> i64 {
    let mut eq_x = (claw_machine.a_x.clone(), claw_machine.b_x.clone(), claw_machine.p_x.clone());
    let mut eq_y = (claw_machine.a_y.clone(), claw_machine.b_y.clone(), claw_machine.p_y.clone());

    // multiply by a values to cancel out
    eq_x = (eq_x.0 * claw_machine.a_y, eq_x.1 * claw_machine.a_y, eq_x.2 * claw_machine.a_y);
    eq_y = (eq_y.0 * claw_machine.a_x, eq_y.1 * claw_machine.a_x, eq_y.2 * claw_machine.a_x);

    // subtract to cancel the a terms
    let comb = (eq_x.1 - eq_y.1, eq_x.2 - eq_y.2);

    // b presses are impossible
    if comb.1 % comb.0 != 0 {
        return 0;
    }

    let press_b = comb.1 / comb.0;

    // a presses are impossible
    if (claw_machine.p_x - (claw_machine.b_x * press_b)) % claw_machine.a_x != 0 {
        return 0;
    }

    let press_a = (claw_machine.p_x - (claw_machine.b_x * press_b)) / claw_machine.a_x;

    return 3*press_a + press_b;


}
