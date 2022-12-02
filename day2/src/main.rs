use std::time::Instant;

const MY_BASELINE: i32 = 'X' as i32;
const OPPONENT_BASELINE: i32 = 'A' as i32;
const SCORES: [i32; 3] = [3, 6, 0];
const DECISIONS: [i32; 3] = [1, 2, 3];

fn main() {
    let input = include_str!("../input.txt");
    part_1(input);
    part_2(input);
}

fn part_1(input: &str) {
    let start = Instant::now();
    let score: i32 = input
        .lines()
        .map(|line| {
            let mut c = line.chars();
            (c.next().unwrap(), c.nth(1).unwrap())
        })
        .fold(0, |total, (opponent, me)| {
            total
                + DECISIONS[(me as i32 - MY_BASELINE) as usize]
                + SCORES[(((me as i32 - MY_BASELINE) - (opponent as i32 - OPPONENT_BASELINE))
                    .rem_euclid(3)) as usize]
        });
    println!("Part 1:{score} in {}us", start.elapsed().as_micros())
}

fn part_2(input: &str) {
    let start = Instant::now();
    let score: i32 = input
        .lines()
        .map(|line| {
            let mut c = line.chars();
            (c.next().unwrap(), c.nth(1).unwrap())
        })
        .fold(0, |total, (opponent, me)| {
            total
                + DECISIONS[((opponent as i32 - OPPONENT_BASELINE) + (me as i32 - MY_BASELINE - 1))
                    .rem_euclid(3) as usize]
                + SCORES[(me as i32 - MY_BASELINE + 2).rem_euclid(3) as usize]
        });
    println!("Part 2:{score} in {}us", start.elapsed().as_micros())
}
