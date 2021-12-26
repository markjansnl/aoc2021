use aoc13::{fold, input, parse_input};

fn print_dots(input: &str) {
    let (mut dots, instructions) = parse_input(input);
    dots = instructions.into_iter().fold(dots, fold);

    let (min_x, min_y, max_x, max_y) = dots.iter().fold(
        (u16::MAX, u16::MAX, 0, 0),
        |(prev_min_x, prev_min_y, prev_max_x, prev_max_y), &(x, y)| {
            (
                prev_min_x.min(x),
                prev_min_y.min(y),
                prev_max_x.max(x),
                prev_max_y.max(y),
            )
        },
    );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if dots.contains(&(x, y)) { "#" } else { " " });
        }
        println!();
    }
}

fn main() {
    print_dots(input::USER);
}

#[test]
fn test_example() {
    print_dots(input::EXAMPLE);
}
