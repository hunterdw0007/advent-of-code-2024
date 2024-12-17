use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::fs::read_to_string;

const TURN_COST: i32 = 1000;
const MOVE_COST: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_count(&self, new_direction: Direction) -> i32 {
        let diff = (*self as i32 - new_direction as i32).abs();
        std::cmp::min(diff, 4 - diff)
    }

    fn get_movement(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn all_directions() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    state: State,
    cost: i32,
    path: Vec<State>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.path.len().cmp(&other.path.len()))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Solution {
    paths: Vec<Vec<State>>,
    cost: i32,
}

struct MazeSolver {
    maze: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl MazeSolver {
    fn new(maze: Vec<Vec<char>>) -> Self {
        let rows = maze.len();
        let cols = maze[0].len();
        MazeSolver { maze, rows, cols }
    }

    fn is_valid_position(&self, x: usize, y: usize) -> bool {
        x < self.cols && y < self.rows && self.maze[y][x] == '.'
    }

    fn get_next_position(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        let (dx, dy) = direction.get_movement();
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        
        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if new_x < self.cols && new_y < self.rows {
                return Some((new_x, new_y));
            }
        }
        None
    }

    fn solve(
        &self,
        start: (usize, usize),
        start_direction: Direction,
        end: (usize, usize),
    ) -> Option<Solution> {
        let start_state = State {
            x: start.0,
            y: start.1,
            direction: start_direction,
        };

        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut solutions = Vec::new();
        let mut min_cost = i32::MAX;

        queue.push(Node {
            state: start_state,
            cost: 0,
            path: vec![start_state],
        });
        visited.insert(start_state);

        while let Some(Node { state, cost, path }) = queue.pop() {
            // If cost is greater than min_cost, we can stop searching
            if cost > min_cost {
                break;
            }

            if (state.x, state.y) == end {
                if cost < min_cost {
                    // Found a better solution, clear previous solutions
                    min_cost = cost;
                    solutions.clear();
                    solutions.push(path);
                } else if cost == min_cost {
                    // Found another solution with same cost
                    solutions.push(path);
                }
                continue;
            }

            for new_direction in Direction::all_directions() {
                if let Some((new_x, new_y)) = self.get_next_position(state.x, state.y, new_direction) {
                    let new_state = State {
                        x: new_x,
                        y: new_y,
                        direction: new_direction,
                    };

                    if !self.is_valid_position(new_x, new_y) {
                        continue;
                    }

                    let turn_cost = if state.direction != new_direction {
                        state.direction.turn_count(new_direction) * TURN_COST
                    } else {
                        0
                    };
                    let new_cost = cost + MOVE_COST + turn_cost;

                    // Only explore if new_cost is not greater than min_cost
                    if new_cost <= min_cost {
                        let mut new_path = path.clone();
                        new_path.push(new_state);

                        // For optimal paths, we might need to revisit nodes
                        // Only skip if we've seen this state with a better cost
                        if !visited.contains(&new_state) {
                            visited.insert(new_state);
                            queue.push(Node {
                                state: new_state,
                                cost: new_cost,
                                path: new_path,
                            });
                        }
                    }
                }
            }
        }

        if solutions.is_empty() {
            None
        } else {
            Some(Solution {
                paths: solutions,
                cost: min_cost,
            })
        }
    }

    fn print_maze_with_path(&self, path: &[State], path_number: usize) {
        let mut display_maze = self.maze.clone();
        
        for (i, state) in path.iter().enumerate() {
            let symbol = match i {
                0 => 'S',
                n if n == path.len() - 1 => 'E',
                _ => match state.direction {
                    Direction::North => '↑',
                    Direction::East => '→',
                    Direction::South => '↓',
                    Direction::West => '←',
                },
            };
            display_maze[state.y][state.x] = symbol;
        }

        println!("\nPath {} of the maze:", path_number + 1);
        for row in &display_maze {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn print_all_solutions(solver: &MazeSolver, solution: &Solution) {
    println!("\nFound {} optimal paths with cost {}:", solution.paths.len(), solution.cost);
    
    for (i, path) in solution.paths.iter().enumerate() {
        println!("\nPath {} details:", i + 1);
        let mut total_moves = 0;
        let mut total_turns = 0;

        for (j, state) in path.iter().enumerate() {
            if j > 0 {
                total_moves += 1;
                let turns = path[j - 1].direction.turn_count(state.direction);
                total_turns += turns;
                
                let turn_str = if turns > 0 {
                    format!(" (turns: {})", turns)
                } else {
                    String::new()
                };
                println!(
                    "({}, {}) facing {:?}{}",
                    state.x, state.y, state.direction, turn_str
                );
            } else {
                println!("({}, {}) facing {:?}", state.x, state.y, state.direction);
            }
        }

        println!("\nPath {} summary:", i + 1);
        println!("Total moves: {}", total_moves);
        println!("Total turns: {}", total_turns);
        solver.print_maze_with_path(path, i);
    }

    // Collect unique positions from all paths
    let unique_positions: HashSet<(usize, usize)> = solution.paths
        .iter()
        .flat_map(|path| path.iter().map(|state| (state.x, state.y)))
        .collect();

    println!("\nTotal unique positions visited across all paths: {}", unique_positions.len());
}

fn main() {
    println!("Day-16");

    let lines = read_to_string("./data.txt").unwrap();

    let mut maze: Vec<Vec<char>> = lines.split('\n').map(|c| c.chars().collect()).collect();

    let start: (usize, usize) = maze.iter()
    .enumerate()
    .find_map(|(i, row)| {
        row.iter()
            .enumerate()
            .find(|(_, &c)| c == 'S')
            .map(|(j, _)| (i, j))
    })
    .unwrap();
    maze[start.0][start.1] = '.';
    println!("{:?}", start);

    let end: (usize, usize) = maze.iter()
    .enumerate()
    .find_map(|(i, row)| {
        row.iter()
            .enumerate()
            .find(|(_, &c)| c == 'E')
            .map(|(j, _)| (i, j))
    })
    .unwrap();
    maze[end.0][end.1] = '.';
    println!("{:?}", end);

    //print_2d_vec(&maze);

    let solver = MazeSolver::new(maze);
    match solver.solve(start, Direction::East, end) {
        Some(solution) => print_all_solutions(&solver, &solution),
        None => println!("No path found!"),
    }

}