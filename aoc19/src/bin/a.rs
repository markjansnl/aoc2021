use aoc19::{input, input_to_scans};

fn count_beacons(input: &str, min_beacons_overlap: usize) -> usize {
    let mut scans = input_to_scans(input);
    let mut result = scans.remove(0);

    'next_scan: while scans.len() > 0 {
        let scan = scans.remove(0);
        for directed_scan in scan.all_directions_iter() {
            if let Some((delta_x, delta_y, delta_z)) =
                directed_scan.overlaps(&result, min_beacons_overlap)
            {
                for (x, y, z) in directed_scan.beacons() {
                    result.add_beacon((x + delta_x, y + delta_y, z + delta_z));
                }
                continue 'next_scan;
            }
        }
        scans.push(scan);
    }

    result.count_beacons()
}

fn main() {
    println!("{}", count_beacons(input::USER, 12));
}

#[test]
fn test_small_example() {
    assert_eq!(7, count_beacons(input::SMALL_EXAMPLE, 6));
}

#[test]
fn test_example() {
    assert_eq!(79, count_beacons(input::EXAMPLE, 12));
}
