use aoc02::input;

pub fn final_depth(input: &str) -> usize {
    let (horizontal, depth, _aim) = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|vec| (vec[0], vec[1].parse::<usize>().unwrap()))
        .fold(
            (0, 0, 0),
            |(horizontal, depth, aim), (command, distance)| match command {
                "forward" => (horizontal + distance, depth + aim * distance, aim),
                "down" => (horizontal, depth, aim + distance),
                "up" => (horizontal, depth, aim - distance),
                _ => unreachable!(),
            },
        );
    horizontal * depth
}

fn main() {
    println!("{}", final_depth(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(900, final_depth(input::EXAMPLE));
}
