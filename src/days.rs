use std::fs::File;
use std::io::{BufReader, Lines};

/* ===================================== DAY 1 ====================================================================== */

pub fn solve_day_1(data: Lines<BufReader<File>>) {
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

/* ===================================== DAY 2 ====================================================================== */

enum Opcode {
    OpcodeAdd = 1,
    OpcodeMult = 2,
    OpcodeHalt = 99,
}

struct Instruction {
    opcode: Opcode,
    input1: usize,
    input2: usize,
    output: usize,
}

fn to_instruction(vec: &Vec<u32>) -> (Instruction, usize) {
    match vec[0] {
        1 => (
            Instruction {
                opcode: Opcode::OpcodeAdd,
                input1: vec[1] as usize,
                input2: vec[2] as usize,
                output: vec[3] as usize,
            },
            4,
        ),
        2 => (
            Instruction {
                opcode: Opcode::OpcodeMult,
                input1: vec[1] as usize,
                input2: vec[2] as usize,
                output: vec[3] as usize,
            },
            4,
        ),
        99 => (
            Instruction {
                opcode: Opcode::OpcodeHalt,
                input1: 0,
                input2: 0,
                output: 0,
            },
            1,
        ),
        _ => panic!(),
    }
}

pub fn solve_day_2(data: Lines<BufReader<File>>) {
    // Turn input strings into a vector of integers.
    let mut data_vec: Vec<u32> = Vec::new();
    for line in data {
        let mut line_vec: Vec<u32> = line
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        data_vec.append(&mut line_vec);
    }

    // Set machine to initial state
    data_vec[1] = 12;
    data_vec[2] = 2;

    let mut pc = 0;

    // Iterate over all ops
    loop {
        let (inst, adv) = to_instruction(&data_vec[pc..].to_vec());

        match inst.opcode {
            Opcode::OpcodeHalt => {
                println!("Value at position 0: {}", data_vec[0]);
                break;
            }
            Opcode::OpcodeAdd => {
                data_vec[inst.output] = data_vec[inst.input1] + data_vec[inst.input2]
            }
            Opcode::OpcodeMult => {
                data_vec[inst.output] = data_vec[inst.input1] * data_vec[inst.input2]
            }
        }

        pc += adv;
    }
}
