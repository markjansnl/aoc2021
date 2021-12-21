use std::panic;

use aoc10::input;

fn error_score(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' => {
                        if stack.pop().unwrap() != '(' {
                            return 3;
                        }
                    }
                    ']' => {
                        if stack.pop().unwrap() != '[' {
                            return 57;
                        }
                    }
                    '}' => {
                        if stack.pop().unwrap() != '{' {
                            return 1197;
                        }
                    }
                    '>' => {
                        if stack.pop().unwrap() != '<' {
                            return 25137;
                        }
                    }
                    _ => panic!("Unknown character"),
                }
            }
            0
        })
        .sum()
}

fn main() {
    println!("{}", error_score(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(26397, error_score(input::EXAMPLE));
}
