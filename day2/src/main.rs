use std::fs::read_to_string;

const FILE_NAME: &str = "input.txt";
const MY_BASELINE: i32 = 'X' as i32;
const OPPONENT_BASELINE: i32 = 'A' as i32;
const SCORES: [i32; 3] = [3, 6, 0];

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let score: i32 = get_input()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.chars().next().unwrap())
                .collect::<Vec<char>>()
                .try_into()
                .unwrap()
        })
        .fold(0, |mut total, [opponent, me]: [char; 2]| {
            total += me as i32 - (MY_BASELINE - 1)
                + SCORES[(((me as i32 - MY_BASELINE) - (opponent as i32 - OPPONENT_BASELINE))
                    .rem_euclid(3)) as usize];
            total
        });
    println!("{score}")
}

fn part_2() {}

fn get_input() -> String {
    read_to_string(FILE_NAME).unwrap()
}
