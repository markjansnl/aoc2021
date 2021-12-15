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
    let mut basin_sizes = Vec::new();
    for y in 0..rows {
        for x in 0..columns {
            let value = matrix.get((y, x)).unwrap();
            if matrix
                .neighbours((y, x), false)
                .filter(|&neighbour| matrix.get(neighbour).unwrap() <= value)
                .next()
                .is_none()
            {
                basin_sizes.push(basin_size(&matrix, HashSet::from([(y, x)])));
            }
        }
    }

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
