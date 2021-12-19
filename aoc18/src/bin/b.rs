use aoc18::{input, SnailfishNumber};

fn max_magnitude(input: &str) -> usize {
    let snailfish_numbers = input
        .lines()
        .map(|line| SnailfishNumber::from(line))
        .collect::<Vec<SnailfishNumber>>();

    let mut max = 0;
    for i in 0..snailfish_numbers.len() {
        for j in 0..snailfish_numbers.len() {
            if i != j {
                let magnitude =
                    (snailfish_numbers[i].clone() + snailfish_numbers[j].clone()).magnitude();
                max = max.max(magnitude);
            }
        }
    }

    max
}

fn main() {
    println!("{}", max_magnitude(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(3993, max_magnitude(input::EXAMPLE));
}
