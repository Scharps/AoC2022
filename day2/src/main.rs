use std::{fs::read_to_string, time::Instant};

const FILE_NAME: &str = "input.txt";
const MY_BASELINE: i32 = 'X' as i32;
const OPPONENT_BASELINE: i32 = 'A' as i32;
const SCORES: [i32; 3] = [3, 6, 0];
const DECISIONS: [i32; 3] = [1, 2, 3];

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let start = Instant::now();
    let score: i32 = get_input()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.chars().next().unwrap())
                .collect::<Vec<char>>()
                .try_into()
                .unwrap()
        })
        .fold(0, |total, [opponent, me]: [char; 2]| {
            total
                + DECISIONS[(me as i32 - MY_BASELINE) as usize]
                + SCORES[(((me as i32 - MY_BASELINE) - (opponent as i32 - OPPONENT_BASELINE))
                    .rem_euclid(3)) as usize]
        });
    println!("Part 1:{score} in {}us", start.elapsed().as_micros())
}

fn part_2() {
    let start = Instant::now();
    let score: i32 = get_input()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.chars().next().unwrap())
                .collect::<Vec<char>>()
                .try_into()
                .unwrap()
        })
        .fold(0, |total, [opponent, me]: [char; 2]| {
            total
                + DECISIONS[((opponent as i32 - OPPONENT_BASELINE) + (me as i32 - MY_BASELINE - 1))
                    .rem_euclid(3) as usize]
                + SCORES[(me as i32 - MY_BASELINE + 2).rem_euclid(3) as usize]
        });
    println!("Part 2:{score} in {}us", start.elapsed().as_micros())
}

fn get_input() -> String {
    read_to_string(FILE_NAME).unwrap()
}
