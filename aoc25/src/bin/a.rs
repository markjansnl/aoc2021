use aoc25::input;

fn last_sea_cucumber_move(input: &str) -> usize {
    let (height, width) =
        input
            .lines()
            .enumerate()
            .fold((0usize, 0), |(height, width), (linenr, line)| {
                if linenr == 0 {
                    (height + 1, line.len())
                } else {
                    (height + 1, width)
                }
            });

    let mut state = input.to_string();
    for step in 1.. {
        let next_state = move_sea_cucumbers(state.clone(), height, width);
        if next_state == state {
            return step;
        } else {
            state = next_state;
        }
    }
    0
}

fn move_sea_cucumbers(state: String, height: usize, width: usize) -> String {
    let mut locations = state
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut east_moves = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if locations[y][x] == '.' && locations[y][(x + width - 1) % width] == '>' {
                east_moves.push((x, y));
            }
        }
    }
    for (x, y) in east_moves {
        locations[y][x] = '>';
        locations[y][(x + width - 1) % width] = '.';
    }

    let mut west_moves = Vec::new();
    for y in (0..height).rev() {
        for x in (0..width).rev() {
            if locations[y][x] == '.' && locations[(y + height - 1) % height][x] == 'v' {
                west_moves.push((x, y));
            }
        }
    }
    for (x, y) in west_moves {
        locations[y][x] = 'v';
        locations[(y + height - 1) % height][x] = '.';
    }

    locations
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    println!("{}", last_sea_cucumber_move(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(58, last_sea_cucumber_move(input::EXAMPLE));
}
