use aoc03::{input, BinNumberList};

fn get_power_consumption(input: &str) -> u32 {
    let list: BinNumberList = input.parse().unwrap();
    let most_common = list.get_most_common();
    let gamma = most_common.to_decimal();
    let epsilon = most_common.not().to_decimal();

    gamma * epsilon
}

fn main() {
    println!("{}", get_power_consumption(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(198, get_power_consumption(input::EXAMPLE));
}
