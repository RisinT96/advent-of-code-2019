use std::fs::File;
use std::io::{BufReader, Lines};

pub fn compute_day_1(data: Lines<BufReader<File>>) -> String {
    let mut fuel: u32 = 0;

    for module in data {
        let mass: u32 = module.unwrap().parse().unwrap();

        fuel += (mass / 3) - 2;
    }

    return fuel.to_string();
}
