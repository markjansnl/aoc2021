use std::collections::HashSet;

pub mod input;

pub type Dot = (u16, u16);
pub type Dots = HashSet<Dot>;
pub type Instruction = (char, u16);
pub type Instructions = Vec<Instruction>;

pub fn parse_input(input: &str) -> (Dots, Instructions) {
    let mut dots = HashSet::new();
    let mut instructions = Vec::new();
    let mut lines = input.lines();
    for line in &mut lines {
        if line == "" {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        dots.insert((x.parse().unwrap(), y.parse().unwrap()));
    }
    for line in lines {
        let (axis, i) = line[11..].split_once("=").unwrap();
        instructions.push((axis.chars().next().unwrap(), i.parse().unwrap()))
    }

    (dots, instructions)
}

pub fn fold(dots: Dots, instruction: Instruction) -> Dots {
    let (axis, i) = instruction;
    dots.into_iter()
        .map(|(x, y)| {
            (
                if axis == 'x' && x > i { 2 * i - x } else { x },
                if axis == 'y' && y > i { 2 * i - y } else { y },
            )
        })
        .collect()
}
