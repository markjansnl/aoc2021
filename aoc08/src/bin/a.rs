use aoc08::input;

fn f(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let io = line.split("|").collect::<Vec<&str>>();
            let output = io[1].trim();
            output
                .split(" ")
                .map(|digit| {
                    let mut chars = digit
                        .chars()
                        .filter(|char| *char != '\n')
                        .collect::<Vec<char>>();
                    chars.sort();
                    chars.into_iter().fold(String::new(), |mut acc, char| {
                        acc.push(char);
                        acc
                    })
                })
                .filter(|str| match str.len() {
                    2 => true,
                    3 => true,
                    4 => true,
                    7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn main() {
    println!("{}", f(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(0, f(input::EXAMPLE));
}
