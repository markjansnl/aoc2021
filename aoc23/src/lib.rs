use pathfinding::prelude::*;
use std::{fmt, iter::repeat};
use AmphipodType::*;

pub mod input;

#[repr(usize)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum AmphipodType {
    A,
    B,
    C,
    D,
    Empty,
}

impl Default for AmphipodType {
    fn default() -> Self {
        AmphipodType::Empty
    }
}

impl From<char> for AmphipodType {
    fn from(c: char) -> Self {
        match c {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            '.' => Empty,
            other => panic!("Wrong input: {other}"),
        }
    }
}

impl AmphipodType {
    fn destination_x(&self) -> usize {
        match *self {
            Empty => 0,
            A => 2,
            B => 4,
            C => 6,
            D => 8,
        }
    }

    fn energy(&self) -> usize {
        match *self {
            Empty => 0,
            A => 1,
            B => 10,
            C => 100,
            D => 1_000,
        }
    }
}

impl fmt::Debug for AmphipodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Situation {
    hallway: [AmphipodType; 7],
    rooms: [Vec<AmphipodType>; 4],
    lines: usize,
}

impl Situation {
    fn new() -> Self {
        Situation {
            hallway: [Empty; 7],
            rooms: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            lines: 2,
        }
    }

    pub fn unfold(&mut self) {
        self.lines = 4;

        self.rooms[0].insert(1, D);
        self.rooms[0].insert(2, D);

        self.rooms[1].insert(1, B);
        self.rooms[1].insert(2, C);

        self.rooms[2].insert(1, A);
        self.rooms[2].insert(2, B);

        self.rooms[3].insert(1, C);
        self.rooms[3].insert(2, A);
    }

    pub fn least_total_energy(&self) -> usize {
        astar(
            self,
            Situation::successors,
            Situation::heuristic,
            Situation::success,
        )
        .map(|(_path, cost)| cost)
        .unwrap_or_default()
    }

    fn successors(&self) -> Vec<(Self, usize)> {
        let direct = self
            .movables(true)
            .into_iter()
            .filter(|(_, amphipod_type)| self.room_open(amphipod_type))
            .map(|((y, x), amphipod_type)| {
                let destination_x = amphipod_type.destination_x();
                if self.hallway_clear(x, destination_x) {
                    let mut next_situation = self.clone();
                    next_situation.remove(y, x);
                    next_situation.rooms[amphipod_type as usize].push(amphipod_type);
                    let cost = (y
                        + x.abs_diff(destination_x)
                        + (self.lines - self.rooms[amphipod_type as usize].len()))
                        * amphipod_type.energy();
                    Some((next_situation, cost))
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        if !direct.is_empty() {
            return direct;
        }

        // If we cannot place an amphid in its destination directly, put them in the hallway
        self.movables(false)
            .into_iter()
            .map(|((y, x), amphipod_type)| {
                [0, 1, 3, 5, 7, 9, 10].into_iter().enumerate().filter_map(
                    move |(index, destination_x)| {
                        if self.hallway_clear(x, destination_x) {
                            let mut next_situation = self.clone();
                            next_situation.remove(y, x);
                            next_situation.hallway[index] = amphipod_type;
                            let cost = (y + x.abs_diff(destination_x)) * amphipod_type.energy();
                            Some((next_situation, cost))
                        } else {
                            None
                        }
                    },
                )
            })
            .flatten()
            .collect()
    }

    fn heuristic(&self) -> usize {
        let mut movables: Vec<((usize, usize), AmphipodType)> = repeat(0)
            .into_iter()
            .zip([0, 1, 3, 5, 7, 9, 10])
            .zip(self.hallway.iter().cloned())
            .filter(|(_, amphipod_type)| amphipod_type != &Empty)
            .collect();

        for (index_x, room_amphipod_type) in [A, B, C, D].into_iter().enumerate() {
            for (index_y, amphipod_type) in self.rooms[index_x].iter().enumerate() {
                if amphipod_type != &room_amphipod_type {
                    movables.push((
                        (self.lines - index_y, (index_x + 1) * 2),
                        amphipod_type.clone(),
                    ));
                }
            }
        }

        movables
            .into_iter()
            .map(|((y, x), amphipod_type)| {
                let destination_x = amphipod_type.destination_x();
                (y + x.abs_diff(destination_x) + 1) * amphipod_type.energy()
            })
            .sum()
    }

    fn success(&self) -> bool {
        self.hallway == [Empty; 7]
            && self.rooms[0]
                .iter()
                .all(|amphipod_type| amphipod_type == &A)
            && self.rooms[1]
                .iter()
                .all(|amphipod_type| amphipod_type == &B)
            && self.rooms[2]
                .iter()
                .all(|amphipod_type| amphipod_type == &C)
            && self.rooms[3]
                .iter()
                .all(|amphipod_type| amphipod_type == &D)
    }

    fn movables(&self, with_hallway: bool) -> Vec<((usize, usize), AmphipodType)> {
        let mut movables: Vec<((usize, usize), AmphipodType)> = if with_hallway {
            repeat(0)
                .into_iter()
                .zip([0, 1, 3, 5, 7, 9, 10])
                .zip(self.hallway.iter().cloned())
                .filter(|(_, amphipod_type)| amphipod_type != &Empty)
                .collect()
        } else {
            vec![]
        };

        for (index, room_amphipod_type) in [A, B, C, D].into_iter().enumerate() {
            if let Some(amphipod_type) = self.rooms[index].clone().pop() {
                if !self.room_open(&room_amphipod_type) {
                    movables.push((
                        (1 + self.lines - self.rooms[index].len(), (index + 1) * 2),
                        amphipod_type,
                    ));
                }
            }
        }

        movables
    }

    fn room_open(&self, amphipod_type: &AmphipodType) -> bool {
        self.rooms[*amphipod_type as usize]
            .iter()
            .all(|room_amphipod_type| room_amphipod_type == amphipod_type)
    }

    fn hallway_clear(&self, start: usize, end: usize) -> bool {
        let range = if start <= end {
            start + 1..end + 1
        } else {
            end..start
        };
        for (x, amphipod_type) in [0, 1, 3, 5, 7, 9, 10].into_iter().zip(self.hallway.iter()) {
            if range.contains(&x) && amphipod_type != &Empty {
                return false;
            }
        }
        true
    }

    fn remove(&mut self, y: usize, x: usize) {
        if y == 0 {
            self.hallway[match x {
                0 => 0,
                1 => 1,
                3 => 2,
                5 => 3,
                7 => 4,
                9 => 5,
                10 => 6,
                _ => unreachable!(),
            }] = Empty;
        } else {
            self.rooms[(x - 2) / 2].pop();
        }
    }
}

impl From<&str> for Situation {
    fn from(input: &str) -> Self {
        let mut situation = Situation::new();
        for line in input.lines().rev().skip(1).take(2) {
            let chars = line.chars().collect::<Vec<char>>();
            for index in 0..4 {
                situation.rooms[index].push(chars[index * 2 + 3].into())
            }
        }
        situation
    }
}

impl fmt::Debug for Situation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{:?}{:?}.{:?}.{:?}.{:?}.{:?}{:?}#",
            self.hallway[0],
            self.hallway[1],
            self.hallway[2],
            self.hallway[3],
            self.hallway[4],
            self.hallway[5],
            self.hallway[6],
        )?;

        for index in 1..=self.lines {
            writeln!(
                f,
                "###{:?}#{:?}#{:?}#{:?}###",
                self.rooms[0]
                    .get(self.lines - index)
                    .cloned()
                    .unwrap_or_default(),
                self.rooms[1]
                    .get(self.lines - index)
                    .cloned()
                    .unwrap_or_default(),
                self.rooms[2]
                    .get(self.lines - index)
                    .cloned()
                    .unwrap_or_default(),
                self.rooms[3]
                    .get(self.lines - index)
                    .cloned()
                    .unwrap_or_default(),
            )?;
        }
        writeln!(f, "  #########")?;
        Ok(())
    }
}
