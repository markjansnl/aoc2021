use aoc05::{input, overlapping_points};

fn main() {
    println!("{}", overlapping_points(input::USER, false));
}

#[test]
fn test_example() {
    assert_eq!(5, overlapping_points(input::EXAMPLE, false));
}
