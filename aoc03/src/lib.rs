use std::{
    ops::{Add, Index},
    str::FromStr,
};

pub mod input;

#[derive(Debug, Clone)]
pub struct BinNumber(Vec<u32>);

impl FromStr for BinNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BinNumber(
            s.bytes()
                .map(|byte| if byte == '0' as u8 { 0 } else { 1 })
                .collect::<Vec<_>>(),
        ))
    }
}

impl BinNumber {
    pub fn new(len: usize) -> BinNumber {
        BinNumber(vec![0; len])
    }

    pub fn not(&self) -> BinNumber {
        BinNumber(self.0.iter().map(|bit| 1 - bit).collect())
    }

    pub fn to_decimal(&self) -> u32 {
        self.0.iter().fold(0, |acc, bit| (acc << 1) + bit)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn map<F>(self, f: F) -> BinNumber
    where
        F: FnMut(u32) -> u32,
    {
        BinNumber(self.0.into_iter().map(f).collect())
    }
}

impl Add for &BinNumber {
    type Output = BinNumber;

    fn add(self, rhs: Self) -> Self::Output {
        BinNumber(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl Index<usize> for BinNumber {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

#[derive(Debug, Clone)]
pub struct BinNumberList(Vec<BinNumber>);

impl FromStr for BinNumberList {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(BinNumberList(
            input.lines().map(|line| line.parse().unwrap()).collect(),
        ))
    }
}

impl BinNumberList {
    pub fn get_most_common(&self) -> BinNumber {
        let number_len = self.0[0].len();
        let list_len = self.0.len() as u32;
        self.0
            .iter()
            .fold(BinNumber::new(number_len), |acc, bin_number| {
                &acc + bin_number
            })
            .map(|count| if 2 * count >= list_len { 1 } else { 0 })
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&BinNumber) -> bool,
    {
        self.0.retain(f)
    }
}

impl Index<usize> for BinNumberList {
    type Output = BinNumber;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
