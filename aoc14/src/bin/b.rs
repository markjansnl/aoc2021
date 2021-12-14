use aoc14::{input, most_minus_least};

fn main() {
    println!("{}", most_minus_least(input::USER, 40));
}

#[test]
fn test_example() {
    assert_eq!(2188189693529, most_minus_least(input::EXAMPLE, 40));
}
