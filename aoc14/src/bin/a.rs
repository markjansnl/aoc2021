use aoc14::{input, most_minus_least};

fn main() {
    println!("{}", most_minus_least(input::USER, 10));
}

#[test]
fn test_example() {
    assert_eq!(1588, most_minus_least(input::EXAMPLE, 10));
}
