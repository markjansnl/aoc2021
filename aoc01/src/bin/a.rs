use aoc01::input;

pub fn count_larger(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .fold((0, u32::MAX), |(count, prev), number| {
            (if number > prev { count + 1 } else { count }, number)
        })
        .0
}

fn main() {
    println!("{}", count_larger(input::USER));
}

#[test]
fn test_example1() {
    assert_eq!(7, count_larger(input::EXAMPLE));
}
