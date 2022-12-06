use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    part_1(input);
    part_2(input);
}

fn part_1(input: &str) {
    let mut result = None;

    for i in 0..(input.len() - 4) {
        if input[i..i + 4].chars().unique().count() == 4 {
            result = Some(i + 4);
            break;
        }
    }
    println!("{result:?}");
}

fn part_2(input: &str) {
    let mut result = None;

    for i in 0..(input.len() - 14) {
        if input[i..i + 14].chars().unique().count() == 14 {
            result = Some(i + 14);
            break;
        }
    }
    println!("{result:?}");
}
