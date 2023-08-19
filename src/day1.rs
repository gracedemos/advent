use std::fs;

pub fn part1() -> u32 {
    let input_data = fs::read_to_string("input/day1.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let mut elves = Vec::new();
    let mut calories = 0;

    for food_item in input_data {
        if let "" = food_item {
            elves.push(calories);
            calories = 0;
        }else {
            calories += food_item.parse::<u32>()
                .unwrap();
        }
    }
    elves.push(calories);

    let mut greatest = elves[0];
    for elf in elves {
        if elf > greatest {
            greatest = elf;
        }
    }

    greatest
}

pub fn part2() -> u32 {
    let input_data = fs::read_to_string("input/day1.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let mut elves = Vec::new();
    let mut calories = 0;

    for food_item in input_data {
        if let "" = food_item {
            elves.push(calories);
            calories = 0;
        }else {
            calories += food_item.parse::<u32>()
                .unwrap();
        }
    }
    elves.push(calories);

    let mut greatest = elves[0];
    let mut elf = 0;
    let mut greatest_elf = 0;

    while elf < elves.len() {
        if elves[elf] > greatest {
            greatest = elves[elf];
            greatest_elf = elf;
        }
        elf += 1;
    }
    elves.remove(greatest_elf);

    let mut second_greatest = elves[0];
    let mut elf = 0;
    let mut greatest_elf = 0;

    while elf < elves.len() {
        if elves[elf] > second_greatest {
            second_greatest = elves[elf];
            greatest_elf = elf;
        }
        elf += 1;
    }
    elves.remove(greatest_elf);

    let mut third_greatest = elves[0];
    let mut elf = 0;
    let mut greatest_elf = 0;

    while elf < elves.len() {
        if elves[elf] > third_greatest {
            third_greatest = elves[elf];
            greatest_elf = elf;
        }
        elf += 1;
    }
    elves.remove(greatest_elf);

    greatest + second_greatest + third_greatest
}