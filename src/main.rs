use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use std::io;

mod days;

fn main() {
    loop {
        println!("Select day: ");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line!");

        /* Check exit options */
        match line.trim() {
            "1" => days::solve_day_1(load_data_file("inputs/1.txt")),
            "2" => days::solve_day_2(load_data_file("inputs/2.txt")),
            "exit" | "q" => break,
            _ => continue,
        };
    }
}

fn load_data_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    return reader.lines();
}
