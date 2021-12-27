use aoc22::{input, parse_input};

fn count_cubes_on(input: &str) -> usize {
    let mut states = [[[false; 101]; 101]; 101];
    for z in -50..=50 {
        for y in -50..=50 {
            for x in -50..=50 {
                for (state, cube) in parse_input(input) {
                    if cube.contains(x, y, z) {
                        states[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize] = state;
                    }
                }
            }
        }
    }
    states
        .iter()
        .flatten()
        .flatten()
        .filter(|state| **state)
        .count()
}

fn main() {
    println!("{}", count_cubes_on(input::USER));
}

#[test]
fn test_example1() {
    assert_eq!(39, count_cubes_on(input::EXAMPLE1));
}

#[test]
fn test_example2() {
    assert_eq!(590784, count_cubes_on(input::EXAMPLE2));
}
