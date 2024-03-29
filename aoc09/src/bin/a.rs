use aoc09::input;
use pathfinding::prelude::Matrix;

fn sum_risk_levels(input: &str) -> usize {
    let (rows, columns, values) = input.lines().fold(
        (0, 0, Vec::new()),
        |(prev_rows, _prev_columns, mut values), line| {
            values.extend(line.bytes().map(|b| b as usize - b'0' as usize));
            (prev_rows + 1, line.len(), values)
        },
    );

    let matrix = Matrix::from_vec(rows, columns, values).unwrap();

    (0..rows)
        .into_iter()
        .map(|y| {
            (0..columns)
                .into_iter()
                .filter_map(|x| {
                    let value = matrix.get((y, x)).unwrap();
                    match matrix
                        .neighbours((y, x), false)
                        .filter(|&neighbour| matrix.get(neighbour).unwrap() <= value)
                        .next()
                    {
                        Some(_) => None,
                        None => Some(value + 1),
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    println!("{}", sum_risk_levels(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(15, sum_risk_levels(input::EXAMPLE));
}
