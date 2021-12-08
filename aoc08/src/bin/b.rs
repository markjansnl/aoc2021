use std::collections::HashSet;

use aoc08::input;

fn sum_output_values(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut io = line.split(" | ").map(|str| {
                str.split(" ")
                    .map(|digit| digit.chars().collect::<HashSet<char>>())
            });
            let signals = io.next().unwrap().collect::<Vec<_>>();
            let output = io.next().unwrap();

            let mut digits = Vec::new();
            (0..10)
                .into_iter()
                .for_each(|_| digits.push(HashSet::new()));

            for signal in signals.clone() {
                match signal.len() {
                    2 => digits[1] = signal,
                    3 => digits[7] = signal,
                    4 => digits[4] = signal,
                    7 => digits[8] = signal,
                    _ => {}
                }
            }

            for signal in signals.clone() {
                match signal.len() {
                    5 => {
                        if signal.intersection(&digits[1]).count() == 2 {
                            digits[3] = signal;
                        } else if signal.intersection(&digits[4]).count() == 2 {
                            digits[2] = signal;
                        } else {
                            digits[5] = signal;
                        }
                    }
                    _ => {}
                }
            }
            for signal in signals {
                match signal.len() {
                    6 => {
                        if signal.intersection(&digits[4]).count() == 4 {
                            digits[9] = signal;
                        } else if signal.intersection(&digits[5]).count() == 5 {
                            digits[6] = signal;
                        } else {
                            digits[0] = signal;
                        }
                    }
                    _ => {}
                }
            }

            output
                .map(|digit| {
                    digits
                        .iter()
                        .enumerate()
                        .find(|(_, digit2)| {
                            digit.intersection(digit2).count() == digit.len().max(digit2.len())
                        })
                        .unwrap()
                        .0
                })
                .fold(0, |acc, digit| acc * 10 + digit)
        })
        .sum()
}

fn main() {
    println!("{}", sum_output_values(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(61229, sum_output_values(input::EXAMPLE));
}

#[test]
fn test_example_short() {
    assert_eq!(
        5353,
        sum_output_values(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        )
    );
}
