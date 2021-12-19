use aoc19::{input, input_to_scans};

fn max_distance(input: &str, min_beacons_overlap: usize) -> usize {
    let mut scans = input_to_scans(input);
    let mut result = scans.remove(0);
    let mut deltas = Vec::new();

    'next_scan: while scans.len() > 0 {
        let scan = scans.remove(0);
        for directed_scan in scan.all_directions_iter() {
            if let Some((delta_x, delta_y, delta_z)) =
                directed_scan.overlaps(&result, min_beacons_overlap)
            {
                deltas.push((delta_x, delta_y, delta_z));
                for (x, y, z) in directed_scan.beacons() {
                    result.add_beacon((x + delta_x, y + delta_y, z + delta_z));
                }
                continue 'next_scan;
            }
        }
        scans.push(scan);
    }

    let mut max = 0;
    for (x1, y1, z1) in &deltas {
        for (x2, y2, z2) in &deltas {
            let distance = (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs();
            max = max.max(distance);
        }
    }
    max as usize
}

fn main() {
    println!("{}", max_distance(input::USER, 12));
}

#[test]
fn test_example() {
    assert_eq!(3621, max_distance(input::EXAMPLE, 12));
}
