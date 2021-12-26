use aoc13::{fold, input, parse_input};

fn count_dots(input: &str) -> usize {
    let (dots, instructions) = parse_input(input);

    let dots = fold(dots, instructions[0]);

    dots.len()
}

fn main() {
    println!("{}", count_dots(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(17, count_dots(input::EXAMPLE));
}
