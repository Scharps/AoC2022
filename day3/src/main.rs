const CAP_START: u8 = b'A';
const LOWER_START: u8 = b'a';

fn main() {
    let input = include_str!("../input.txt");
    part_1(input);
}

fn part_1(input: &str) {
    let total = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .fold(0, |total: u32, (a, b)| {
            for c in a.chars() {
                if b.contains(c) {
                    let c = c as u8;
                    return match c {
                        65..=90 => total + (c - CAP_START + 1) as u32 + 26,
                        97..=122 => total + (c - LOWER_START + 1) as u32,
                        _ => panic!(),
                    };
                }
            }
            total
        });
    println!("{total}")
}
