use std::fs;
use std::process;

pub fn part1() -> u32 {
    let input_data = fs::read_to_string("input/day6.txt")
        .unwrap();
    let input_data: Vec<char> = input_data.chars()
        .collect();

    for i in 0..input_data.len() {
        let mut chars = Vec::new();

        for j in i..(i + 4) {
            chars.push(input_data[j]);
        }

        if chars[0] != chars[1] && chars[0] != chars[2] && chars[0] != chars[3] {
            if chars[1] != chars[0] && chars[1] != chars[2] && chars[1] != chars[3] {
                if chars[2] != chars[0] && chars[2] != chars[1] && chars[2] != chars[3] {
                    if chars[3] != chars[0] && chars[3] != chars[1] && chars[3] != chars[2] {
                        return i as u32 + 4;
                    }
                }
            }
        }
    }
    
    eprintln!("Failed to find start signal");
    process::exit(1);
}

pub fn part2() -> u32 {
    let input_data = fs::read_to_string("input/day6.txt")
        .unwrap();
    let input_data: Vec<char> = input_data.chars()
        .collect();

    for i in 0..input_data.len() {
        let mut chars = Vec::new();

        for j in i..(i + 14) {
            chars.push(input_data[j]);
        }

        let mut correct = true;
        'outer: for j in 0..chars.len() {
            for k in 0..chars.len() {
                if j != k {
                    if chars[j] == chars[k] {
                        correct = false;
                        break 'outer;
                    }
                }
            }
        }

        if correct {
            return i as u32 + 14;
        }
    }

    eprintln!("Failed to find start signal");
    process::exit(1);
}
