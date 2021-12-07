use std::collections::BTreeMap;

use aoc07::input;

#[derive(Debug)]
struct Position {
    count: usize,
    count_before: usize,
    count_after: usize,
    fuel_before: usize,
    fuel_after: usize,
}

fn min_fuel(input: &str) -> usize {
    let (map, max, initial_count_after, initial_fuel_after) = input.split(",").fold(
        (BTreeMap::new(), 0, 0, 0),
        |(mut map, max, count_after, fuel_after), number| {
            let number = number.parse().unwrap();
            map.entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            (map, max.max(number), count_after + 1, fuel_after + number)
        },
    );

    let mut min_fuel = usize::MAX;
    let initial_count = *map.get(&0).unwrap_or(&0);
    let init = Position {
        count: initial_count,
        count_before: 0,
        count_after: initial_count_after - initial_count,
        fuel_before: 0,
        fuel_after: initial_fuel_after,
    };

    (1..=max).into_iter().fold(init, |previous, number| {
        let count = *map.get(&number).unwrap_or(&0);
        let position = Position {
            count,
            count_before: previous.count_before + previous.count,
            count_after: previous.count_after - count,
            fuel_before: previous.fuel_before + previous.count_before + previous.count,
            fuel_after: previous.fuel_after - previous.count_after,
        };
        min_fuel = min_fuel.min(position.fuel_before + position.fuel_after);
        position
    });
    min_fuel
}

fn main() {
    println!("{}", min_fuel(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(37, min_fuel(input::EXAMPLE));
}
