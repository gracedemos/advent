use std::fs;

pub fn part1() -> String {
    let input_data = fs::read_to_string("input/day5.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let mut stacks = get_stacks(&input_data, 9, 8);

    for line in input_data {
        if !line.contains("move") {
            continue;
        }

        do_stack_move(line, &mut stacks);
    }

    let mut ret = String::new();
    for stack in stacks {
        ret.push(*stack.last()
                 .unwrap());
    }

    ret
}

pub fn part2() -> String {
    let input_data = fs::read_to_string("input/day5.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let mut stacks = get_stacks(&input_data, 9, 8);

    for line in input_data {
        if !line.contains("move") {
            continue;
        }

        do_stack_move2(line, &mut stacks);
    }

    let mut ret = String::new();
    for stack in stacks {
        ret.push(*stack.last()
                 .unwrap());
    }

    ret
}

fn get_stacks(input_data: &Vec<&str>, stack_count: usize, height: usize) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    for i in 0..height {
        let line: Vec<char> = input_data[i].chars()
            .collect();
        let mut i = 1;
        let mut stack_i = 0;
        while i < line.len() {
            if line[i] != ' ' {
                stacks[stack_i].push(line[i]);
            }
            i += 4;
            stack_i += 1;
        }
    }

    for i in 0..stacks.len() {
        stacks[i].reverse();
    }

    stacks
}

fn do_stack_move(operation: &str, stacks: &mut Vec<Vec<char>>) {
    let (n1, n2, n3) = get_nums(operation);

    for _ in 0..n1 {
        let val = stacks[n2 as usize - 1].pop()
            .unwrap();
        stacks[n3 as usize - 1].push(val);
    }
}

fn do_stack_move2(operation: &str, stacks: &mut Vec<Vec<char>>) {
    let (n1, n2, n3) = get_nums(operation);

    let mut vals = Vec::new();
    for _ in 0..n1 {
        let val = stacks[n2 as usize - 1].pop()
            .unwrap();
        vals.push(val);
    }

    for _ in 0..vals.len() {
        let val = vals.pop()
            .unwrap();
        stacks[n3 as usize - 1].push(val);
    }
}

fn get_nums(operation: &str) -> (u32, u32, u32) {
    let op_chars: Vec<char> = operation.chars()
        .collect();

    if op_chars[6] != ' ' {
        let n1 = operation[5..7].parse::<u32>()
            .unwrap();
        let n2 = op_chars[13].to_digit(10)
            .unwrap();
        let n3 = op_chars[18].to_digit(10)
            .unwrap();

        (n1, n2, n3)
    }else {
        let n1 = op_chars[5].to_digit(10)
            .unwrap();
        let n2 = op_chars[12].to_digit(10)
            .unwrap();
        let n3 = op_chars[17].to_digit(10)
            .unwrap();

        (n1, n2, n3)
    }
}
