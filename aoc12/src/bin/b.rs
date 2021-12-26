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
    let mut paths = vec![(vec!["start"], false)];

    while !paths.is_empty() {
        let mut next_paths = Vec::new();
        for (path, twice) in &paths {
            let last = path.last().unwrap();
            for destination in connections.get(last).unwrap() {
                let small_cave = &destination.to_lowercase().as_str() == destination;
                if *destination == "start" {
                    // do nothing
                } else if *destination == "end" {
                    count += 1;
                } else if !small_cave || !twice || !path.contains(destination) {
                    let mut new_path = path.clone();
                    new_path.push(destination);
                    next_paths.push((
                        new_path,
                        *twice || (small_cave && path.contains(destination)),
                    ));
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
    assert_eq!(36, path_count(input::EXAMPLE1));
}

#[test]
fn test_example2() {
    assert_eq!(103, path_count(input::EXAMPLE2));
}

#[test]
fn test_example3() {
    assert_eq!(3509, path_count(input::EXAMPLE3));
}
