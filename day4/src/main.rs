use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");
    part_1(input);
    part_2(input);
}

fn part_1(input: &str) {
    let start = Instant::now();
    let total = input
        .lines()
        .map(|line| {
            let mut elves = line.split(',').map(|jobs_str| {
                let mut bounds = jobs_str
                    .split('-')
                    .map(|range| str::parse::<usize>(range).unwrap());

                (bounds.next().unwrap(), bounds.next().unwrap())
            });
            (elves.next().unwrap(), elves.next().unwrap())
        })
        .fold(0, |total, (a, b)| {
            if (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1) {
                return total + 1;
            }
            total
        });
    println!("Part 1:{total} in {}us", start.elapsed().as_micros())
}

fn part_2(input: &str) {
    let start = Instant::now();
    let total = input
        .lines()
        .map(|line| {
            let mut elves = line.split(',').map(|jobs_str| {
                let mut bounds = jobs_str
                    .split('-')
                    .map(|range| str::parse::<usize>(range).unwrap());

                (bounds.next().unwrap(), bounds.next().unwrap())
            });
            (elves.next().unwrap(), elves.next().unwrap())
        })
        .fold(0, |total, (a, b)| {
            if !(a.0 > b.1 || b.0 > a.1) && !(a.1 < b.0 || b.1 < a.0) {
                return total + 1;
            }
            total
        });
    println!("Part 2:{total} in {}us", start.elapsed().as_micros())
}
