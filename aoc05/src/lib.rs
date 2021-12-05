use std::collections::HashMap;

pub mod input;

pub fn overlapping_points(input: &str, diagonals: bool) -> usize {
    let mut coordinates = HashMap::new();
    for line in input.lines() {
        let split = line
            .split(" -> ")
            .map(|coord| coord.split(",").map(|a| a.parse().unwrap()))
            .flatten()
            .collect::<Vec<usize>>();
        let (x1, y1, x2, y2) = (split[0], split[1], split[2], split[3]);

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                coordinates
                    .entry((x1, y))
                    .and_modify(|t| *t += 1)
                    .or_insert(1usize);
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                coordinates
                    .entry((x, y1))
                    .and_modify(|t| *t += 1)
                    .or_insert(1usize);
            }
        } else if diagonals {
            for (i, x) in (x1.min(x2)..=x1.max(x2)).enumerate() {
                let y = if (x1 < x2 && y1 < y2) || (x1 > x2 && y1 > y2) {
                    y1.min(y2) + i
                } else {
                    y1.max(y2) - i
                };
                coordinates
                    .entry((x, y))
                    .and_modify(|t| *t += 1)
                    .or_insert(1usize);
            }
        }
    }

    coordinates.values().filter(|t| **t > 1).count()
}
