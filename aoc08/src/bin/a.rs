use aoc08::input;

fn sum_output_values(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let io = line.split(" | ").collect::<Vec<&str>>();
            io[1]
                .split(" ")
                .filter(|str| match str.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn main() {
    println!("{}", sum_output_values(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(26, sum_output_values(input::EXAMPLE));
}
