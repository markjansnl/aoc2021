use aoc23::{Situation, input};

fn main() {
    let mut situation = Situation::from(input::USER);
    situation.unfold();
    println!("{}", situation.least_total_energy());
}

#[test]
fn test_example() {
    let mut situation = Situation::from(input::EXAMPLE);
    situation.unfold();
    assert_eq!(44169, situation.least_total_energy());
}
