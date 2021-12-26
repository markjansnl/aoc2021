use std::collections::HashMap;

use aoc12::input;

fn path_count(input: &str) -> usize {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        if let Some((from, to)) = line.split_once("-") {
            connections
                .entry(from)
                .and_modify(|destinations| destinations.push(to))
                .or_insert(vec![to]);
            connections
                .entry(to)
                .and_modify(|destinations| destinations.push(from))
                .or_insert(vec![from]);
        }
    }

    let mut count = 0;
    let mut paths = vec![vec!["start"]];

    while !paths.is_empty() {
        let mut next_paths = Vec::new();
        for path in &paths {
            let last = path.last().unwrap();
            for destination in connections.get(last).unwrap() {
                if *destination == "end" {
                    count += 1;
                } else if &destination.to_lowercase().as_str() != destination
                    || !path.contains(destination)
                {
                    let mut new_path = path.clone();
                    new_path.push(destination);
                    next_paths.push(new_path);
                }
            }
        }
        paths = next_paths;
    }

    count
}

fn main() {
    println!("{}", path_count(input::USER));
}

#[test]
fn test_example1() {
    assert_eq!(10, path_count(input::EXAMPLE1));
}

#[test]
fn test_example2() {
    assert_eq!(19, path_count(input::EXAMPLE2));
}

#[test]
fn test_example3() {
    assert_eq!(226, path_count(input::EXAMPLE3));
}
