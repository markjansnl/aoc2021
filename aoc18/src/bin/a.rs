use aoc18::{input, SnailfishNumber};

fn magnitude(input: &str) -> usize {
    input
        .lines()
        .map(|line| SnailfishNumber::from(line))
        .sum::<SnailfishNumber>()
        .magnitude()
}

fn main() {
    println!("{}", magnitude(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(4140, magnitude(input::EXAMPLE));
}
