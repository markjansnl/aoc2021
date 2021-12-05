use aoc05::{input, overlapping_points};

fn main() {
    println!("{}", overlapping_points(input::USER, true));
}

#[test]
fn test_example() {
    assert_eq!(12, overlapping_points(input::EXAMPLE, true));
}
