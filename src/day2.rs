use std::fs;
use std::process;

enum RoundOutcome {
    Victory,
    Loss,
    Draw
}

pub fn part1() -> u32 {
    let input_data = fs::read_to_string("input/day2.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let mut total_score = 0;
    for round in input_data {
        total_score += get_score(round);
    }

    total_score
}

pub fn part2() -> u32 {
    let input_data = fs::read_to_string("input/day2.txt")
        .unwrap();
    let input_data: Vec<&str> = input_data.lines()
        .collect();

    let mut total_score = 0;
    for round in input_data {
        total_score += get_score2(round);
    }

    total_score
}

fn get_score2(round: &str) -> u32 {
    let mut score = 0;
    let moves: Vec<char> = round.chars()
        .collect();

    match moves[2] {
        'X' => { // Lose
            match moves[0] {
                'A' => { // Rock
                    score += 3;
                },
                'B' => { // Paper
                    score += 1;
                },
                'C' => { // Scissors
                    score += 2;
                },
                _ => ()
            }
        },
        'Y' => { // Draw
            score += 3;
            match moves[0] {
                'A' => { // Rock
                    score += 1;
                },
                'B' => { // Paper
                    score += 2;
                },
                'C' => { // Scissors
                    score += 3;
                },
                _ => ()
            }
        },
        'Z' => { // Win
            score += 6;
            match moves[0] {
                'A' => { // Rock
                    score += 2;
                },
                'B' => { // Paper
                    score += 3;
                },
                'C' => { // Scissors
                    score += 1;
                },
                _ => ()
            }
        },
        _ => ()
    }

    score
}

fn get_score(round: &str) -> u32 {
    let mut score = 0;
    let moves: Vec<char> = round.chars()
        .collect();

    match moves[2] {
        'X' => { // Rock
            score += 1;
        },
        'Y' => { // Paper
            score += 2;
        },
        'Z' => { // Scissors
            score += 3;
        },
        _ => ()
    }

    match get_outcome(moves[0], moves[2]) {
        RoundOutcome::Victory => {
            score += 6;
        },
        RoundOutcome::Draw => {
            score += 3;
        },
        RoundOutcome::Loss => {}
    }

    score
}

fn get_outcome(enemy: char, you: char) -> RoundOutcome {
    match enemy {
        'A' => { // Rock
            match you {
                'X' => { // Rock
                    return RoundOutcome::Draw;
                },
                'Y' => { // Paper
                    return RoundOutcome::Victory;
                },
                'Z' => { // Scissors
                    return RoundOutcome::Loss;
                },
                _ => ()
            }
        },
        'B' => { // Paper
            match you {
                'X' => { // Rock
                    return RoundOutcome::Loss;
                },
                'Y' => { // Paper
                    return RoundOutcome::Draw;
                },
                'Z' => { // Scissors
                    return RoundOutcome::Victory;
                },
                _ => ()
            }
        },
        'C' => { // Scissors
            match you {
                'X' => { // Rock
                    return RoundOutcome::Victory;
                },
                'Y' => { // Paper
                    return RoundOutcome::Loss;
                },
                'Z' => { // Scissors
                    return RoundOutcome::Draw;
                },
                _ => ()
            }
        },
        _ => ()
    }

    eprintln!("Improper input");
    process::exit(1);
}