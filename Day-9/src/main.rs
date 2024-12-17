use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    println!("Day 9");

    let mut raw_data: Vec<char> = Vec::new();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            raw_data = line.chars().collect();
        }
    }

    let total_len: u32 = raw_data.iter().map(|x| x.to_digit(10).unwrap()).sum();

    let mut data: Vec<Option<usize>> = vec![];
    data.reserve(total_len as usize);

    fill_data(&mut data, &raw_data);

    while !move_data(&mut data) {

    }

    println!("Checksum Part 1: {}", data.iter().enumerate().map(|x| x.0 * x.1.unwrap()).sum::<usize>());

    let mut tuple_data: Vec<(usize, Option<usize>)> = vec![];
    tuple_data.reserve(raw_data.len());

    let max_id = fill_tuples(&mut tuple_data, &raw_data);

    for i in (0..max_id).rev() {
        move_data_chunks(&mut tuple_data, i);
        combine_consecutive_none(&mut tuple_data);
    }
    print_tuples(&tuple_data);

    println!("Checksum Part 2: {}", checksum_tuple(&tuple_data));

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

fn fill_data(data: &mut Vec<Option<usize>>, raw_data: &Vec<char>) {
    let mut id = 0;
    for (i, elem) in raw_data.iter().enumerate() {
        let chunk_len = elem.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            data.append(&mut vec![Some(id); chunk_len]);
            id += 1;
        } else {
            data.append(&mut vec![None; chunk_len]);
        }
    }
}

fn move_data(data: &mut Vec<Option<usize>>) -> bool {
    let first_open = data.iter().position(|x| x.is_none()).unwrap();
    let last_data = data.len() - 1 - data.iter().rev().position(|x| x.is_some()).unwrap();

    if first_open > last_data {
        data.drain(first_open..);
        return true;
    }

    data.swap(first_open, last_data);
    return false;
}

fn print_tuples(tuples: &Vec<(usize, Option<usize>)>) {
    for &tuple in tuples {
        match tuple.1 {
            Some(s) => print!("{}", s.to_string().repeat(tuple.0)),
            None => print!("{}", " ".repeat(tuple.0)),
        }
    }
    println!();
}

fn fill_tuples(data: &mut Vec<(usize, Option<usize>)>, raw_data: &Vec<char>) -> usize {
    let mut id = 0;

    for (i, elem) in raw_data.iter().enumerate() {
        let chunk_len = elem.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            data.push((chunk_len, Some(id)));
            id += 1;
        }
        else {
            data.push((chunk_len, None));
        }
    }
    id
}


// only want to go once through the data in reverse
fn move_data_chunks(data: &mut Vec<(usize, Option<usize>)>, file_id: usize) {

    // find the tuple for the file_id i'm processing
    let file_loc = data.iter().position(|&f| f.1 == Some(file_id)).unwrap();
    // find the first open place to the left that it can fit in
    let open_loc = data.iter().enumerate().position(|(loc, &f)| loc < file_loc && f.0 >= data[file_loc].0 && f.1 == None );
    match open_loc {
        None => return (), // there are no available locations to put the file
        Some(valid_loc) => {
            // get the data we want to move
            let file_proc = data.remove(file_loc);
            // replace it with empty space
            data.insert(file_loc, (file_proc.0, None));
            // insert it before the open space
            data.insert(valid_loc, file_proc);
            // modify the open space
            let space = data.get_mut(valid_loc + 1).unwrap();
            space.0 -= file_proc.0;
        }
    }


}

fn checksum_tuple(data: &Vec<(usize, Option<usize>)>) -> usize {
    let mut cs = 0;
    let mut index = 0; 

    for &tuple in data {
        match tuple.1 {
            Some(id) => {
                for i in index..index + tuple.0 {
                    cs += index * id;
                    index += 1;
                }
            },
            None => {
                index += tuple.0;
            },
        }
        
    }
    cs
}

fn combine_consecutive_none(data: &mut Vec<(usize, Option<usize>)>) {
    let mut read_idx = 0;
    let mut write_idx = 0;
    let mut current_sum = 0;
    let mut combining = false;

    while read_idx < data.len() {
        match data[read_idx].1 {
            None => {
                current_sum += data[read_idx].0;
                combining = true;
            }
            Some(val) => {
                if combining {
                    data[write_idx] = (current_sum, None);
                    write_idx += 1;
                    current_sum = 0;
                    combining = false;
                }
                data[write_idx] = (data[read_idx].0, Some(val));
                write_idx += 1;
            }
        }
        read_idx += 1;
    }

    if combining {
        data[write_idx] = (current_sum, None);
        write_idx += 1;
    }

    data.truncate(write_idx);
}