use std::collections::HashMap;

use aoc04::input;

#[derive(Debug)]
struct Board {
    numbers: HashMap<usize, (usize, usize)>,
    marked: [[bool; 5]; 5],
}

fn first_bingo(input: &str) -> usize {
    let mut lines = input.lines();
    let draws = lines
        .next()
        .unwrap()
        .split(",")
        .map(|draw| draw.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        let mut board = Board {
            numbers: HashMap::new(),
            marked: [[false; 5]; 5],
        };
        for y in 0..5usize {
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .enumerate()
                .for_each(|(x, number)| {
                    let number = number.parse::<usize>().unwrap();
                    board.numbers.insert(number, (x, y));
                });
        }
        boards.push(board);
    }

    for draw in draws {
        for board in boards.iter_mut() {
            if let Some(&(x, y)) = board.numbers.get(&draw) {
                board.marked[y][x] = true;

                if board.marked[y] == [true; 5]
                    || (board.marked[0][x]
                        && board.marked[1][x]
                        && board.marked[2][x]
                        && board.marked[3][x]
                        && board.marked[4][x])
                {
                    return draw
                        * board
                            .numbers
                            .iter()
                            .filter_map(|(&number, &(x2, y2))| {
                                (!board.marked[y2][x2]).then(|| number)
                            })
                            .sum::<usize>();
                }
            }
        }
    }

    0
}

fn main() {
    println!("{}", first_bingo(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(4512, first_bingo(input::EXAMPLE));
}
