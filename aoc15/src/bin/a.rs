use aoc15::{fastest_path, input};

fn total_risk(input: &str) -> usize {
    let (rows, columns, values) = input.lines().fold(
        (0, 0, Vec::new()),
        |(prev_rows, _prev_columns, mut values), line| {
            values.extend(line.bytes().map(|b| b as usize - b'0' as usize));
            (prev_rows + 1, line.len(), values)
        },
    );

    fastest_path(rows, columns, values)
}

fn main() {
    println!("{}", total_risk(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(40, total_risk(input::EXAMPLE));
}
