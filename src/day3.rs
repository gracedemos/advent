use std::fs;
use std::process;

pub fn part1() -> u32 {
    let input_data = fs::read_to_string("input/day3.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let mut total_priority = 0;
    for sack in input_data {
        let c = get_sack_duplicate(sack);
        total_priority += get_priority(c);
    }

    total_priority
}

pub fn part2() -> u32 {
    let input_data = fs::read_to_string("input/day3.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let mut total_priority = 0;
    let mut index = 0;
    while index < input_data.len() {
        let c = get_triple_duplicate(&input_data, index);
        total_priority += get_priority(c);
        index += 3;
    }

    total_priority
}

fn get_triple_duplicate(input_data: &Vec<&str>, index: usize) -> char {
    for c1 in input_data[index].chars() {
        for c2 in input_data[index + 1].chars() {
            if c1 == c2 {
                for c3 in input_data[index + 2].chars() {
                    if c2 == c3 {
                        return c3;
                    }
                }
            }
        }
    }

    eprintln!("Failed to find duplicate");
    process::exit(1);
}

fn get_sack_duplicate(sack: &str) -> char {
    let middle_index = sack.len() / 2;
    let compartment1 = &sack[..middle_index];
    let compartment2 = &sack[middle_index..];

    for c1 in compartment1.chars() {
        for c2 in compartment2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }

    eprintln!("No duplicates found");
    process::exit(1);
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        return (c as u32) - 96;
    }

    (c as u32) - 38
}