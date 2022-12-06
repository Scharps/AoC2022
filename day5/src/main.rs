use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    part_1(input);
    part_2(input);
}

fn part_1(input: &str) {
    let mut sections = input.split("\n\n");
    let mut stacks: Vec<VecDeque<char>> = vec![];
    sections.next().unwrap().lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if stacks.len() == ((i as i32 - 1) / 4) as usize {
                stacks.push(VecDeque::default());
            }
            if (i as isize - 1) % 4 == 0 && c.is_alphabetic() {
                stacks[((i as i32 - 1) / 4) as usize].push_back(c);
            }
        })
    });
    let mut buff: VecDeque<char> = VecDeque::default();
    sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            (
                iter.nth(1).unwrap().parse::<usize>().unwrap(),
                iter.nth(1).unwrap().parse::<usize>().unwrap(),
                iter.nth(1).unwrap().parse::<usize>().unwrap(),
            )
        })
        .for_each(|(quantity, from, to)| {
            while buff.len() < quantity {
                buff.push_back(stacks[from - 1].pop_front().unwrap());
            }
            while !buff.is_empty() {
                stacks[to - 1].push_front(buff.pop_front().unwrap())
            }
        });
    println!("{stacks:?}")
}

fn part_2(input: &str) {
    let mut sections = input.split("\n\n");
    let mut stacks: Vec<VecDeque<char>> = vec![];
    sections.next().unwrap().lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if stacks.len() == ((i as i32 - 1) / 4) as usize {
                stacks.push(VecDeque::default());
            }
            if (i as isize - 1) % 4 == 0 && c.is_alphabetic() {
                stacks[((i as i32 - 1) / 4) as usize].push_back(c);
            }
        })
    });
    let mut buff: VecDeque<char> = VecDeque::default();
    sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            (
                iter.nth(1).unwrap().parse::<usize>().unwrap(),
                iter.nth(1).unwrap().parse::<usize>().unwrap(),
                iter.nth(1).unwrap().parse::<usize>().unwrap(),
            )
        })
        .for_each(|(quantity, from, to)| {
            while buff.len() < quantity {
                buff.push_back(stacks[from - 1].pop_front().unwrap());
            }
            while !buff.is_empty() {
                stacks[to - 1].push_front(buff.pop_back().unwrap())
            }
        });
    println!("{stacks:?}")
}
