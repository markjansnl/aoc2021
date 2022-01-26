use std::fmt::Display;

use aoc24::alu;

#[derive(Default, Clone)]
struct ModelNumber([u8; 14]);

impl ModelNumber {
    pub fn new(number: i64) -> Self {
        let mut number = number;
        let mut model_number = ModelNumber::default();

        for digit in model_number.0.iter_mut().rev() {
            *digit = (number % 10) as u8;
            number /= 10;
        }

        model_number
    }

    pub fn accepted(&self) -> bool {
        let alu_result = alu::user(self.0.iter().map(|digit| *digit as i64));
        alu_result.z == 0
    }

    pub fn decrease(&mut self) {
        let mut index = 13;
        loop {
            self.0[index] -= 1;
            if self.0[index] > 0 || index == 0 {
                break;
            }
            self.0[index] = 9;
            index -= 1;
        }
    }
}

impl Display for ModelNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.0.iter() {
            let _ = write!(f, "{}", digit);
        }
        Ok(())
    }
}

fn largest_accepted_model_number() -> ModelNumber {
    let mut model_number = ModelNumber::new(99_999_999_999_999);
    let mut x = 0;

    while !model_number.accepted() {
        model_number.decrease();
        x += 1;
        if x > 1_000_000_000 {
            println!("{}", model_number);
            x = 0;
        }
    }

    model_number
}

fn main() {
    println!("{}", largest_accepted_model_number());
}
