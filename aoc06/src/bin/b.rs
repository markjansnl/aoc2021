use aoc06::{input, count_fish};

fn main() {
    println!("{}", count_fish(input::USER, 256));
}

#[test]
fn test_example() {
    assert_eq!(26984457539, count_fish(input::EXAMPLE, 256));
}
