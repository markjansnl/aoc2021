use aoc23::input;
use pathfinding::prelude::*;
use std::fmt;
use AmphipodType::*;
use Place::*;

#[derive(PartialEq, Eq, Hash, Clone)]
enum AmphipodType {
    Empty,
    A,
    B,
    C,
    D,
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
            _ => panic!("Wrong input!"),
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

const HALLWAY_1: usize = 0;
const HALLWAY_2: usize = 1;
const HALLWAY_3: usize = 2;
const HALLWAY_4: usize = 3;
const HALLWAY_5: usize = 4;
const HALLWAY_6: usize = 5;
const HALLWAY_7: usize = 6;
const ROOM_A_NORTH: usize = 7;
const ROOM_A_SOUTH: usize = 8;
const ROOM_B_NORTH: usize = 9;
const ROOM_B_SOUTH: usize = 10;
const ROOM_C_NORTH: usize = 11;
const ROOM_C_SOUTH: usize = 12;
const ROOM_D_NORTH: usize = 13;
const ROOM_D_SOUTH: usize = 14;

#[repr(usize)]
#[derive(Clone)]
enum Place {
    Hallway1,
    Hallway2,
    Hallway3,
    Hallway4,
    Hallway5,
    Hallway6,
    Hallway7,
    RoomANorth,
    RoomASouth,
    RoomBNorth,
    RoomBSouth,
    RoomCNorth,
    RoomCSouth,
    RoomDNorth,
    RoomDSouth,
}

impl From<usize> for Place {
    fn from(index: usize) -> Self {
        use Place::*;
        match index {
            HALLWAY_1 => Hallway1,
            HALLWAY_2 => Hallway2,
            HALLWAY_3 => Hallway3,
            HALLWAY_4 => Hallway4,
            HALLWAY_5 => Hallway5,
            HALLWAY_6 => Hallway6,
            HALLWAY_7 => Hallway7,
            ROOM_A_NORTH => RoomANorth,
            ROOM_A_SOUTH => RoomASouth,
            ROOM_B_NORTH => RoomBNorth,
            ROOM_B_SOUTH => RoomBSouth,
            ROOM_C_NORTH => RoomCNorth,
            ROOM_C_SOUTH => RoomCSouth,
            ROOM_D_NORTH => RoomDNorth,
            ROOM_D_SOUTH => RoomDSouth,
            _ => panic!("Wrong place!"),
        }
    }
}

impl Place {
    fn position(&self) -> (usize, usize) {
        match *self {
            Hallway1 => (0, 0),
            Hallway2 => (0, 1),
            Hallway3 => (0, 3),
            Hallway4 => (0, 5),
            Hallway5 => (0, 7),
            Hallway6 => (0, 9),
            Hallway7 => (0, 10),
            RoomANorth => (1, 2),
            RoomASouth => (2, 2),
            RoomBNorth => (1, 4),
            RoomBSouth => (2, 4),
            RoomCNorth => (1, 6),
            RoomCSouth => (2, 6),
            RoomDNorth => (1, 8),
            RoomDSouth => (2, 8),
        }
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone)]
struct Situation([AmphipodType; 15]);

const GOAL: Situation = {
    Situation([
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, A, A, B, B, C, C, D, D,
    ])
};

impl From<&str> for Situation {
    fn from(input: &str) -> Self {
        let mut situation = Self::default();
        for (y, line) in input.lines().skip(2).take(2).enumerate() {
            let chars = line.chars().collect::<Vec<char>>();
            for char_index in [3, 5, 7, 9] {
                situation.0[char_index + 4 + y] = chars[char_index].into();
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
            self.0[HALLWAY_1],
            self.0[HALLWAY_2],
            self.0[HALLWAY_3],
            self.0[HALLWAY_4],
            self.0[HALLWAY_5],
            self.0[HALLWAY_6],
            self.0[HALLWAY_7]
        )?;
        writeln!(
            f,
            "###{:?}#{:?}#{:?}#{:?}###",
            self.0[ROOM_A_NORTH], self.0[ROOM_B_NORTH], self.0[ROOM_C_NORTH], self.0[ROOM_D_NORTH]
        )?;
        writeln!(
            f,
            "  #{:?}#{:?}#{:?}#{:?}#",
            self.0[ROOM_A_SOUTH], self.0[ROOM_B_SOUTH], self.0[ROOM_C_SOUTH], self.0[ROOM_D_SOUTH]
        )?;
        writeln!(f, "  #########")?;
        Ok(())
    }
}

impl Situation {
    fn successors(&self) -> Vec<(Self, usize)> {
        let direct = self
            .movables(false)
            .filter(|(_, amphipod_type)| self.room_open(amphipod_type))
            .map(|(place, amphipod_type)| {
                let (y, x) = place.position();
                let destination_x = amphipod_type.destination_x();

                if self.hallway_clear(x, destination_x) {
                    let mut next_situation = self.clone();
                    next_situation.0[place as usize] = Empty;
                    next_situation.0[self.room_destination_first(&amphipod_type)] =
                        amphipod_type.clone();
                    let cost =
                        (y + x.abs_diff(destination_x) + self.room_destination_y(amphipod_type))
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
            .filter(|(place, _)| {
                place.clone() as usize > HALLWAY_7
            })
            .map(|(place, amphipod_type)| {
                let (y, x) = place.position();
                (0..7).filter_map(move |hallway_place| {
                    let hallway_place = Place::from(hallway_place);
                    let (_, destination_x) = hallway_place.position();
                    if self.hallway_clear(x, destination_x) {
                        let mut next_situation = self.clone();
                        next_situation.0[place.clone() as usize] = Empty;
                        next_situation.0[hallway_place as usize] = amphipod_type.clone();
                        let cost = (y + x.abs_diff(destination_x)) * amphipod_type.energy();
                        Some((next_situation, cost))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    fn room_open(&self, amphipod_type: &AmphipodType) -> bool {
        match amphipod_type {
            A => {
                self.0[ROOM_A_NORTH] == Empty
                    && (self.0[ROOM_A_SOUTH] == Empty || self.0[ROOM_A_SOUTH] == A)
            }
            B => {
                self.0[ROOM_B_NORTH] == Empty
                    && (self.0[ROOM_B_SOUTH] == Empty || self.0[ROOM_B_SOUTH] == B)
            }
            C => {
                self.0[ROOM_C_NORTH] == Empty
                    && (self.0[ROOM_C_SOUTH] == Empty || self.0[ROOM_C_SOUTH] == C)
            }
            D => {
                self.0[ROOM_D_NORTH] == Empty
                    && (self.0[ROOM_D_SOUTH] == Empty || self.0[ROOM_D_SOUTH] == D)
            }
            _ => unreachable!(),
        }
    }

    fn room_destination_y(&self, amphipod_type: &AmphipodType) -> usize {
        match amphipod_type {
            A => {
                if self.0[ROOM_A_SOUTH] == Empty {
                    2
                } else {
                    1
                }
            }
            B => {
                if self.0[ROOM_B_SOUTH] == Empty {
                    2
                } else {
                    1
                }
            }
            C => {
                if self.0[ROOM_C_SOUTH] == Empty {
                    2
                } else {
                    1
                }
            }
            D => {
                if self.0[ROOM_D_SOUTH] == Empty {
                    2
                } else {
                    1
                }
            }
            _ => unreachable!(),
        }
    }

    fn room_destination_first(&self, amphipod_type: &AmphipodType) -> usize {
        match amphipod_type {
            A => {
                if self.0[ROOM_A_SOUTH] == Empty {
                    ROOM_A_SOUTH
                } else {
                    ROOM_A_NORTH
                }
            }
            B => {
                if self.0[ROOM_B_SOUTH] == Empty {
                    ROOM_B_SOUTH
                } else {
                    ROOM_B_NORTH
                }
            }
            C => {
                if self.0[ROOM_C_SOUTH] == Empty {
                    ROOM_C_SOUTH
                } else {
                    ROOM_C_NORTH
                }
            }
            D => {
                if self.0[ROOM_D_SOUTH] == Empty {
                    ROOM_D_SOUTH
                } else {
                    ROOM_D_NORTH
                }
            }
            _ => unreachable!(),
        }
    }

    fn hallway_clear(&self, start: usize, end: usize) -> bool {
        let range = if start <= end {
            start + 1..end + 1
        } else {
            end..start
        };
        for (place, amphipod_type) in self.0[..7].iter().enumerate() {
            if range.contains(&Place::from(place).position().1) && amphipod_type != &Empty {
                return false;
            }
        }
        true
    }

    fn heuristic(&self) -> usize {
        self.movables(true)
            .map(|(place, amphipod_type)| {
                let (y, x) = place.position();
                let destination_x = amphipod_type.destination_x();
                (y + x.abs_diff(destination_x) + 1) * amphipod_type.energy()
            })
            .sum()
    }

    fn movables(&self, with_locked: bool) -> impl Iterator<Item = (Place, &AmphipodType)> {
        self.0
            .iter()
            .enumerate()
            .filter(move |(place, amphipod_type)| {
                amphipod_type != &&Empty
                    && (amphipod_type != &&GOAL.0[*place] || match *place {
                        ROOM_A_NORTH => self.0[ROOM_A_SOUTH] != GOAL.0[ROOM_A_SOUTH],
                        ROOM_B_NORTH => self.0[ROOM_B_SOUTH] != GOAL.0[ROOM_B_SOUTH],
                        ROOM_C_NORTH => self.0[ROOM_C_SOUTH] != GOAL.0[ROOM_C_SOUTH],
                        ROOM_D_NORTH => self.0[ROOM_D_SOUTH] != GOAL.0[ROOM_D_SOUTH],
                        _ => false
                    })                                        
                    && (with_locked || {
                        match *place {
                            ROOM_A_SOUTH => self.0[ROOM_A_NORTH] == Empty,
                            ROOM_B_SOUTH => self.0[ROOM_B_NORTH] == Empty,
                            ROOM_C_SOUTH => self.0[ROOM_C_NORTH] == Empty,
                            ROOM_D_SOUTH => self.0[ROOM_D_NORTH] == Empty,
                            _ => true,
                        }
                    })
            })
            .map(|(place, amphipod_type)| (Place::from(place), amphipod_type))
    }

    fn success(&self) -> bool {
        self == &GOAL
    }
}

fn least_total_energy(input: &str) -> usize {
    astar(
        &Situation::from(input),
        Situation::successors,
        Situation::heuristic,
        Situation::success,
    )
    .map(|(_path, cost)| cost)
    .unwrap_or_default()
}

fn main() {
    println!("{}", least_total_energy(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(12521, least_total_energy(input::EXAMPLE));
}
