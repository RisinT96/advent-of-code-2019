use std::fs::File;
use std::io::{BufReader, Lines};

pub fn compute_day_1(data: Lines<BufReader<File>>) {
    fn compute_fuel(mass: u32) -> u32 {
        use std::cmp::max;

        return max((mass as i32 / 3) - 2, 0) as u32;
    }

    let mut total_fuel_pt1: u32 = 0;
    let mut total_fuel_pt2: u32 = 0;

    for module in data {
        let mass: u32 = module.unwrap().parse().unwrap();

        let module_fuel = compute_fuel(mass);

        total_fuel_pt1 += module_fuel;
        total_fuel_pt2 += module_fuel;

        let mut additional_fuel = compute_fuel(module_fuel);

        while additional_fuel > 0 {
            total_fuel_pt2 += additional_fuel;
            additional_fuel = compute_fuel(additional_fuel);
        }
    }
    println!("Part One: {}", total_fuel_pt1);

    println!("Part Two: {}", total_fuel_pt2);
}
