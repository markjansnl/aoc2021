use std::{array::IntoIter, str::Bytes};

pub struct BitIterator<'a> {
    head: Option<IntoIter<bool, 3>>,
    tail: Bytes<'a>,
    bits_read: usize,
}

impl From<&'static str> for BitIterator<'static> {
    fn from(input: &'static str) -> Self {
        BitIterator {
            head: None,
            tail: input.bytes(),
            bits_read: 0,
        }
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head) = self.head.as_mut() {
            if let Some(bit) = head.next() {
                self.bits_read += 1;
                return Some(bit);
            }
        }

        if let Some(hex) = self.tail.next() {
            let (bit, head) = match hex {
                b'0' => (false, [false, false, false]),
                b'1' => (false, [false, false, true]),
                b'2' => (false, [false, true, false]),
                b'3' => (false, [false, true, true]),
                b'4' => (false, [true, false, false]),
                b'5' => (false, [true, false, true]),
                b'6' => (false, [true, true, false]),
                b'7' => (false, [true, true, true]),
                b'8' => (true, [false, false, false]),
                b'9' => (true, [false, false, true]),
                b'A' => (true, [false, true, false]),
                b'B' => (true, [false, true, true]),
                b'C' => (true, [true, false, false]),
                b'D' => (true, [true, false, true]),
                b'E' => (true, [true, true, false]),
                b'F' => (true, [true, true, true]),
                _ => unreachable!(),
            };
            self.head = Some(head.into_iter());
            self.bits_read += 1;
            Some(bit)
        } else {
            None
        }
    }
}

impl<'a> BitIterator<'a> {
    pub fn take_number(&mut self, bits: usize) -> usize {
        let mut number = 0;
        for _ in 0..bits {
            number = (number << 1) + if self.next().unwrap() { 1 } else { 0 };
        }
        number
    }

    pub fn bits_read(&self) -> usize {
        self.bits_read
    }
}
