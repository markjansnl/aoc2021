use std::collections::BTreeMap;

use aoc07::input;

fn min_fuel(input: &str) -> i32 {
    let (map, max) = input
        .split(",")
        .fold((BTreeMap::new(), 0i32), |(mut map, max), number| {
            let number = number.parse().unwrap();
            map.entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            (map, max.max(number))
        });

    (0..=max)
        .into_iter()
        .map(|number| {
            map.iter()
                .map(|(number2, count)| {
                    let distance = (number - number2).abs();
                    count * distance * (distance + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap_or_default()
}

fn main() {
    println!("{}", min_fuel(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(168, min_fuel(input::EXAMPLE));
}
