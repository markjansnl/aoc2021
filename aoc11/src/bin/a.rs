use aoc11::{flash, input};

fn flash_count(input: &str, steps: usize) -> usize {
    let mut flashes = 0;
    let mut levels = input
        .lines()
        .flat_map(|line| line.bytes().map(|b| b - b'0'))
        .collect::<Vec<u8>>();

    for _ in 0..steps {
        for level in levels.iter_mut() {
            *level += 1;
        }

        for i in 0..levels.len() {
            flashes += flash(&mut levels, i);
        }
    }

    flashes
}

fn main() {
    println!("{}", flash_count(input::USER, 100));
}

#[test]
fn test_example() {
    assert_eq!(1656, flash_count(input::EXAMPLE, 100));
}
