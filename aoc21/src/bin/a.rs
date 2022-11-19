use aoc21::input;

struct Game {
    pos: [u8; 2],
    score: [u16; 2],
}

impl Game {
    pub fn new(input: &str) -> Game {
        let mut pos = input.lines().map(|line| line[28..].parse::<u8>().unwrap());
        Game {
            pos: [pos.next().unwrap() - 1, pos.next().unwrap() - 1],
            score: [0; 2],
        }
    }

    pub fn play(&mut self, die: impl Iterator<Item = u8>) -> usize {
        let mut die_with_index = die.enumerate();
        let mut player = 0;
        while self.score.iter().find(|&&score| score >= 1000).is_none() {
            self.pos[player] = [
                die_with_index.next().unwrap(),
                die_with_index.next().unwrap(),
                die_with_index.next().unwrap(),
            ]
            .into_iter()
            .fold(self.pos[player], |prev_pos, (_, moves)| {
                (prev_pos + moves) % 10
            });
            self.score[player] += (self.pos[player] + 1) as u16;
            player = 1 - player;
        }
        *self.score.iter().min().unwrap() as usize * die_with_index.next().unwrap().0
    }
}

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
