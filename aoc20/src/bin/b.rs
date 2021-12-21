use aoc20::{count_lit_pixels, input};

fn main() {
    println!("{}", count_lit_pixels(input::USER, 50));
}

#[test]
fn test_example() {
    assert_eq!(3351, count_lit_pixels(input::EXAMPLE, 50));
}
