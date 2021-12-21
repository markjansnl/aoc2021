use std::panic;

use aoc10::input;

fn middle_score(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' => {
                        if stack.pop().unwrap() != '(' {
                            return None;
                        }
                    }
                    ']' => {
                        if stack.pop().unwrap() != '[' {
                            return None;
                        }
                    }
                    '}' => {
                        if stack.pop().unwrap() != '{' {
                            return None;
                        }
                    }
                    '>' => {
                        if stack.pop().unwrap() != '<' {
                            return None;
                        }
                    }
                    _ => panic!("Unknown character"),
                }
            }
            Some(stack.into_iter().rev().fold(0, |score, c| {
                score * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    }
            }))
        })
        .collect::<Vec<usize>>();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    println!("{}", middle_score(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(288957, middle_score(input::EXAMPLE));
}
