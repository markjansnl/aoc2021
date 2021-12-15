use std::collections::HashSet;

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

    let mut basin_sizes = (0..rows)
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
                        None => Some(basin_size(&matrix, HashSet::from([(y, x)]))),
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    basin_sizes.sort();
    basin_sizes.reverse();

    basin_sizes.into_iter().take(3).product()
}

fn basin_size(matrix: &Matrix<usize>, set: HashSet<(usize, usize)>) -> usize {
    let mut set2 = set.clone();
    for &p in set.iter() {
        set2.extend(
            matrix
                .neighbours(p, false)
                .filter(|&neighbour| *matrix.get(neighbour).unwrap() < 9),
        );
    }
    if set2.len() == set.len() {
        set.len()
    } else {
        basin_size(matrix, set2)
    }
}

fn main() {
    println!("{}", sum_risk_levels(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(1134, sum_risk_levels(input::EXAMPLE));
}
