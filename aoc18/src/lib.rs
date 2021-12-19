use std::{iter::Sum, ops::Add, str::Bytes};

pub mod input;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SnailfishNumber {
    RegularNumber(u8),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl From<&str> for SnailfishNumber {
    fn from(s: &str) -> Self {
        let mut bytes = s.bytes();
        SnailfishNumber::parse(&mut bytes)
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = SnailfishNumber::new_pair(self, rhs);

        loop {
            let mut mut_ref_vec = result.as_mut_ref_vec(0);
            if let Some((index, (number, _))) = mut_ref_vec
                .iter()
                .enumerate()
                .find(|(_, (_, depth))| *depth > 4)
            {
                if index > 0 {
                    *mut_ref_vec[index - 1].0 += **number;
                }
                if index < mut_ref_vec.len() - 2 {
                    *mut_ref_vec[index + 2].0 += *mut_ref_vec[index + 1].0;
                }
                result.explode(0);
                continue;
            }

            if result.split() {
                continue;
            }

            break;
        }
        result
    }
}

impl Sum for SnailfishNumber {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let first = iter.next().unwrap();
        iter.fold(first, Add::add)
    }
}

impl SnailfishNumber {
    pub fn new_regular_number(regular_number: u8) -> Self {
        SnailfishNumber::RegularNumber(regular_number)
    }

    pub fn new_pair(left: SnailfishNumber, right: SnailfishNumber) -> Self {
        SnailfishNumber::Pair(Box::new(left), Box::new(right))
    }

    fn parse(bytes: &mut Bytes) -> Self {
        let start = bytes.next().unwrap();
        if start == b'[' {
            let left = SnailfishNumber::parse(bytes);
            assert_eq!(b',', bytes.next().unwrap());
            let right = SnailfishNumber::parse(bytes);
            assert_eq!(b']', bytes.next().unwrap());
            SnailfishNumber::new_pair(left, right)
        } else {
            SnailfishNumber::new_regular_number(start - b'0')
        }
    }

    fn as_mut_ref_vec(&mut self, depth: u8) -> Vec<(&mut u8, u8)> {
        let mut vec = Vec::new();
        match self {
            SnailfishNumber::RegularNumber(regular_number) => vec.push((regular_number, depth)),
            SnailfishNumber::Pair(left, right) => {
                vec.append(&mut left.as_mut_ref_vec(depth + 1));
                vec.append(&mut right.as_mut_ref_vec(depth + 1));
            }
        };
        vec
    }

    fn explode(&mut self, depth: u8) -> bool {
        if let SnailfishNumber::Pair(left, right) = self {
            if depth == 4 {
                *self = SnailfishNumber::new_regular_number(0);
                true
            } else {
                left.explode(depth + 1) || right.explode(depth + 1)
            }
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailfishNumber::RegularNumber(ref regular_number) => {
                if *regular_number > 9 {
                    let left_regular_number = regular_number / 2;
                    let right_regular_number = regular_number - left_regular_number;
                    *self = SnailfishNumber::new_pair(
                        SnailfishNumber::new_regular_number(left_regular_number),
                        SnailfishNumber::new_regular_number(right_regular_number),
                    );
                    true
                } else {
                    false
                }
            }
            SnailfishNumber::Pair(left, right) => left.split() || right.split(),
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            SnailfishNumber::RegularNumber(regular_number) => *regular_number as usize,
            SnailfishNumber::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}
