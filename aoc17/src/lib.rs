pub mod input;

#[derive(Debug, PartialEq, Eq)]
pub enum Hit {
    Above,
    Hit,
    Under,
    UnderShot,
    OverShot,
}

#[derive(Debug)]
pub struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl From<&str> for TargetArea {
    fn from(input: &str) -> Self {
        let (x_range, y_range) = input[15..].split_once(", y=").unwrap();
        let (min_x, max_x) = x_range.split_once("..").unwrap();
        let (min_y, max_y) = y_range.split_once("..").unwrap();
        TargetArea {
            min_x: min_x.parse().unwrap(),
            max_x: max_x.parse().unwrap(),
            min_y: min_y.parse().unwrap(),
            max_y: max_y.parse().unwrap(),
        }
    }
}

impl TargetArea {
    pub fn hit(&self, x: i32, y: i32) -> Hit {
        if y < self.min_y {
            Hit::Under
        } else if y > self.max_y {
            Hit::Above
        } else if x < self.min_x {
            Hit::UnderShot
        } else if x > self.max_x {
            Hit::OverShot
        } else {
            Hit::Hit
        }
    }
}

pub struct Trajectory {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Iterator for Trajectory {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        self.velocity_x -= self.velocity_x.signum();
        self.velocity_y -= 1;

        Some((self.x, self.y))
    }
}

impl Trajectory {
    pub fn new(velocity_x: i32, velocity_y: i32) -> Self {
        Trajectory {
            x: 0,
            y: 0,
            velocity_x,
            velocity_y,
        }
    }
}
