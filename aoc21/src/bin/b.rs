use std::collections::HashMap;

use aoc21::input;

const MIN_SCORE: usize = 21;

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    /** Positions on the board, zero-based */
    pos: [usize; 2],

    /** Scores of the two players */
    score: [usize; 2],

    /** Player who's turn is next */
    player: usize,

    /** Universes in which we can arrive in this state */
    universes: usize,
}

impl State {
    fn winning_player(&self) -> Option<usize> {
        if self.score[0] >= MIN_SCORE {
            Some(0)
        } else if self.score[1] >= MIN_SCORE {
            Some(1)
        } else {
            None
        }
    }
}

fn most_universes(input: &str) -> usize {
    // Parse positions from input
    let mut pos_iter = input
        .lines()
        .map(|line| line[28..].parse::<usize>().unwrap());

    // Initial state
    let state = State {
        pos: [pos_iter.next().unwrap() - 1, pos_iter.next().unwrap() - 1],
        score: [0; 2],
        player: 0,
        universes: 1,
    };

    // Clean state cache
    let mut state_cache = HashMap::new();

    // Go into the recursive function to decide in how many universes the players won
    let (player1_won, player2_won) = won_universes(&state, &mut state_cache);

    // Return in how many universes the best player won
    player1_won.max(player2_won)
}

fn won_universes(
    state: &State,
    state_cache: &mut HashMap<State, (usize, usize)>,
) -> (usize, usize) {
    // Sounds familiair? If this state is already in the cache, we know the answer. Return it
    if let Some(winnings) = state_cache.get(state) {
        return *winnings;
    }

    // Is this a winning state, then return the universes for that player
    if let Some(winning_player) = state.winning_player() {
        return if winning_player == 0 {
            (state.universes, 0)
        } else {
            (0, state.universes)
        };
    }

    // If we didn't win yet, get the results from the next 9 states from 27 universes, in the following distribution
    let (player1_won, player2_won) = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
        .iter()
        .fold((0, 0), |(player1_won, player2_won), (steps, universes)| {
            let mut next_state = state.clone();
            next_state.pos[state.player] = (next_state.pos[state.player] + steps) % 10;
            next_state.score[state.player] += next_state.pos[state.player] + 1;
            next_state.player = 1 - next_state.player;
            next_state.universes *= universes;
            let (next_player1_won, next_player2_won) = won_universes(&next_state, state_cache);
            (
                player1_won + next_player1_won,
                player2_won + next_player2_won,
            )
        });

    // Insert the result in the cache for later
    state_cache.insert(state.clone(), (player1_won, player2_won));

    // And return the result
    (player1_won, player2_won)
}

fn main() {
    println!("{}", most_universes(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(444356092776315, most_universes(input::EXAMPLE));
}
