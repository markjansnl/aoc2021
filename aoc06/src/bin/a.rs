use aoc06::{input, count_fish};

fn main() {
    println!("{}", count_fish(input::USER, 80));
}

#[test]
fn test_example() {
    assert_eq!(5934, count_fish(input::EXAMPLE, 80));
}
