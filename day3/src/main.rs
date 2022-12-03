use std::time::Instant;

const CAP_START: u8 = b'A';
const LOWER_START: u8 = b'a';

fn main() {
    let input = include_str!("../input.txt");
    part_1(input);
    part_2(input);
}

fn part_1(input: &str) {
    let start = Instant::now();
    let total = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .fold(0, |total: u32, (a, b)| {
            for c in a.chars() {
                if b.contains(c) {
                    return total + score_char(c);
                }
            }
            total
        });
    println!("Part 1:{total} in {}us", start.elapsed().as_micros());
}

fn part_2(input: &str) {
    let start = Instant::now();
    let total =
        input
            .lines()
            .enumerate()
            .fold((0, ["", "", ""]), |(total, mut concat), (index, line)| {
                let elf_n = index % 3;
                concat[elf_n] = line;
                if elf_n == 2 {
                    if let Some(c) = concat[0]
                        .chars()
                        .chain(concat[1].chars())
                        .chain(concat[2].chars())
                        .find(|c| {
                            concat[0].contains(*c)
                                && concat[1].contains(*c)
                                && concat[2].contains(*c)
                        })
                    {
                        return (total + score_char(c), concat);
                    }
                }
                (total, concat)
            });
    println!("Part 2:{} in {}us", total.0, start.elapsed().as_micros());
}

fn score_char(c: char) -> u32 {
    let c_num = c as u8;
    match c {
        'A'..='Z' => (c_num - CAP_START + 1) as u32 + 26,
        'a'..='z' => (c_num - LOWER_START + 1) as u32,
        _ => panic!(),
    }
}
