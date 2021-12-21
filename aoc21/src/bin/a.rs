use aoc21::{input, Game};

fn f(input: &str) -> usize {
    let mut game = Game::new(input);
    let die = (1..=100).cycle();
    game.play(die)
}

fn main() {
    println!("{}", f(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(739785, f(input::EXAMPLE));
}
