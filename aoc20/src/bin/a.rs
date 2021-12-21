use aoc20::{count_lit_pixels, input};

fn main() {
    println!("{}", count_lit_pixels(input::USER, 2));
}

#[test]
fn test_example() {
    assert_eq!(35, count_lit_pixels(input::EXAMPLE, 2));
}
