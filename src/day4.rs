use std::fs;

struct Bounds {
    low: u32,
    high: u32
}

struct Pair {
    first: Bounds,
    second: Bounds
}

pub fn part1() -> u32 {
    let input_data = fs::read_to_string("input/day4.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let pairs = input_data.iter()
        .map(|line| {
            let pair: Vec<&str> = line.split(",")
                .collect();

            (pair[0], pair[1])
        })
    .collect::<Vec<(&str, &str)>>();

    let pairs = pairs.iter()
        .map(|pair| {
            let first = pair.0.split("-")
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap()
                })
            .collect::<Vec<u32>>();
            let first = Bounds { low: first[0], high: first[1] };

            let second = pair.1.split("-")
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap()
                })
            .collect::<Vec<u32>>();
            let second = Bounds { low: second[0], high: second[1] };

            Pair { first, second }
        })
    .collect::<Vec<Pair>>();

    let mut contained = 0;
    for pair in pairs {
        if pair.first.low >= pair.second.low && pair.first.high <= pair.second.high {
            contained += 1;
        }
        else if pair.second.low >= pair.first.low && pair.second.high <= pair.first.high {
            contained += 1;
        }
    }

    contained
}

pub fn part2() -> u32 {
    let input_data = fs::read_to_string("input/day4.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let pairs = input_data.iter()
        .map(|line| {
            let pair: Vec<&str> = line.split(",")
                .collect();

            (pair[0], pair[1])
        })
    .collect::<Vec<(&str, &str)>>();

    let pairs = pairs.iter()
        .map(|pair| {
            let first = pair.0.split("-")
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap()
                })
            .collect::<Vec<u32>>();
            let first = Bounds { low: first[0], high: first[1] };

            let second = pair.1.split("-")
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap()
                })
            .collect::<Vec<u32>>();
            let second = Bounds { low: second[0], high: second[1] };

            Pair { first, second }
        })
    .collect::<Vec<Pair>>();

    let mut overlap = 0;
    for pair in pairs {
        if pair.first.low >= pair.second.low && pair.first.low <= pair.second.high {
            overlap += 1;
        }
        else if pair.first.high <= pair.second.high && pair.first.high >= pair.second.low {
            overlap += 1;
        }
        else if pair.second.low >= pair.first.low && pair.second.low <= pair.first.high {
            overlap += 1;
        }
        else if pair.second.high <= pair.first.high && pair.second.high >= pair.first.low {
            overlap += 1;
        }
    }

    overlap
}
