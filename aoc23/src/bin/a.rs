use aoc23::{Situation, input};

fn main() {
    let situation = Situation::from(input::USER);
    println!("{}", situation.least_total_energy());
}

#[test]
fn test_example() {
    let situation = Situation::from(input::EXAMPLE);
    assert_eq!(12521, situation.least_total_energy());
}
