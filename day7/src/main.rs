use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let mut dir_stack: Vec<&str> = vec![];
    let mut dir_size: HashMap<&str, usize> = HashMap::new();
    input
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .for_each(|mut arg_iter| match arg_iter.next().unwrap() {
            "$" => match arg_iter.next().unwrap() {
                "cd" => {
                    let cd_arg = arg_iter.next().unwrap();
                    match cd_arg {
                        ".." => {
                            dir_stack.pop();
                        }
                        _ => dir_stack.push(cd_arg),
                    }
                }
                "ls" => {}
                x => panic!("Some unhandled case {x}"),
            },
            x => {
                if x.chars().all(|c| c.is_numeric()) {
                    let size = x.parse::<usize>().unwrap();
                    for dir in dir_stack.iter() {
                        dir_size
                            .entry(dir)
                            .and_modify(|s| *s += size)
                            .or_insert(size);
                    }
                }
            }
        });
    println!(
        "{}",
        dir_size.values().filter(|s| **s <= 100_000).sum::<usize>()
    );
}
