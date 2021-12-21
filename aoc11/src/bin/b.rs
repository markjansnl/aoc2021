use aoc11::{flash, input};

fn full_flash(input: &str) -> usize {
    let mut levels = input
        .lines()
        .flat_map(|line| line.bytes().map(|b| b - b'0'))
        .collect::<Vec<u8>>();

    for step in 0.. {
        for level in levels.iter_mut() {
            *level += 1;
        }

        for i in 0..levels.len() {
            flash(&mut levels, i);
        }

        if levels.iter().filter(|&&level| level == 0).count() == 100 {
            return step + 1;
        }
    }
    0
}

fn main() {
    println!("{}", full_flash(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(195, full_flash(input::EXAMPLE));
}
