use aoc15::{fastest_path, input};

fn total_risk(input: &str) -> usize {
    let (rows, columns, values) = input.lines().fold(
        (0, 0, Vec::new()),
        |(prev_rows, _prev_columns, mut values_tile), line| {
            for x in 0..5 {
                values_tile.extend(
                    line.bytes()
                        .map(|b| ((b as usize - b'0' as usize + x - 1) % 9) + 1),
                );
            }
            (prev_rows + 5, line.len() * 5, values_tile)
        },
    );

    let mut values2 = Vec::with_capacity(rows * columns);
    for y in 0..5 {
        values2.extend(values.iter().map(|v| ((*v + y - 1) % 9) + 1));
    }

    fastest_path(rows, columns, values2)
}

fn main() {
    println!("{}", total_risk(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(315, total_risk(input::EXAMPLE));
}
