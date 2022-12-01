use std::{fs::read_to_string, time::Instant};

const FILE_PATH: &str = "input.txt";

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let now = Instant::now();
    let result = get_input()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|cal_str| str::parse::<u32>(cal_str).unwrap())
                .sum::<u32>()
        })
        .fold(0, |max, next| {
            if next > max {
                return next;
            }
            max
        });
    println!("Part 1:{result} in {}us", now.elapsed().as_micros())
}

fn part_2() {
    let now = Instant::now();
    let result: u32 = get_input()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|cal_str| str::parse::<u32>(cal_str).unwrap())
                .sum::<u32>()
        })
        .fold([0, 0, 0], |mut highest, mut next| {
            for candidate in highest.iter_mut() {
                if next > *candidate {
                    std::mem::swap(&mut (*candidate), &mut next);
                }
            }
            highest
        })
        .iter()
        .sum();
    println!("Part 2:{result} in {}us", now.elapsed().as_micros())
}

fn get_input() -> String {
    read_to_string(FILE_PATH).unwrap()
}
