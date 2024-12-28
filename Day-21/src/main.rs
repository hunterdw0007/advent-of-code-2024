use std::fs::read_to_string;

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;

const ITERATIONS: usize = 2;

struct Pad {
    grid: Grid,
    start_pos: Position,
}

impl Pad {
    fn new(grid: Grid) -> Self {
        let start_pos = Self::find_position(&grid, 'A')
            .expect("Pad must contain starting position 'A'");
        Self { grid, start_pos }
    }

    fn find_position(grid: &Grid, target: char) -> Option<Position> {
        grid.iter().enumerate()
            .find_map(|(i, row)| {
                row.iter().enumerate()
                    .find(|(_, &val)| val == target)
                    .map(|(j, _)| (i, j))
            })
    }

    fn generate_moves(&self, code: &[char]) -> Vec<char> {
        let mut path = Vec::new();
        let mut current = self.start_pos;

        for &c in code {
            let target = Self::find_position(&self.grid, c)
                .unwrap_or_else(|| panic!("Invalid character in code: {}", c));
            
            self.append_path(&mut path, current, target);
            path.push('A');
            current = target;
        }

        path
    }

    fn append_path(&self, path: &mut Vec<char>, start: Position, end: Position) {
        let (mut curr_i, mut curr_j) = start;
        let (end_i, end_j) = end;

        // Vertical movement
        while curr_i != end_i {
            path.push(if curr_i > end_i { '^' } else { 'v' });
            curr_i = if curr_i > end_i { curr_i - 1 } else { curr_i + 1 };
        }

        // Horizontal movement
        while curr_j != end_j {
            path.push(if curr_j > end_j { '<' } else { '>' });
            curr_j = if curr_j > end_j { curr_j - 1 } else { curr_j + 1 };
        }
    }
}

fn process_paths(input: &str, num_pad: &Pad, dir_pad: &Pad) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| {
            let code: Vec<char> = line.chars().collect();
            let mut moves = num_pad.generate_moves(&code);
            
            // Apply dir_pad moves repeatedly
            for _ in 0..ITERATIONS {
                moves = dir_pad.generate_moves(&moves);
            }
            moves
        })
        .collect()
}

fn calculate_sum(input: &str, paths: &[Vec<char>]) -> usize {
    input.lines()
        .zip(paths)
        .map(|(line, path)| {
            let num = line[..3].parse::<usize>()
                .unwrap_or_else(|_| panic!("Invalid number in input: {}", &line[..3]));
            num * path.len()
        })
        .sum()
}

fn main() {
    println!("Day 21");

    let input = read_to_string("./data.txt")
        .expect("Failed to read input file");

    let num_pad = Pad::new(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A'],
    ]);

    let dir_pad = Pad::new(vec![
        vec!['#', '^', 'A'],
        vec!['<', 'v', '>'],
    ]);

    let paths = process_paths(&input, &num_pad, &dir_pad);

    // Print paths and their lengths
    paths.iter()
        .for_each(|p| println!("Path: {:?} with length {}", p.iter().cloned().collect::<String>(), p.len()));

    let sum = calculate_sum(&input, &paths);
    println!("Sum: {}", sum);
}