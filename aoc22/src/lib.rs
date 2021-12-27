pub mod input;

#[derive(Debug)]
pub struct Cube {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub min_z: i32,
    pub max_z: i32,
}

pub fn parse_input(input: &str) -> Vec<(bool, Cube)> {
    input
        .lines()
        .map(|line| {
            let (state, cube_str) = line.split_once(" ").unwrap();
            let coordinates = cube_str
                .split(",")
                .map(|c| c[2..].split_once("..").unwrap())
                .collect::<Vec<_>>();
            let cube = Cube {
                min_x: coordinates[0].0.parse().unwrap(),
                max_x: coordinates[0].1.parse().unwrap(),
                min_y: coordinates[1].0.parse().unwrap(),
                max_y: coordinates[1].1.parse().unwrap(),
                min_z: coordinates[2].0.parse().unwrap(),
                max_z: coordinates[2].1.parse().unwrap(),
            };

            assert!(cube.min_x <= cube.max_x);
            assert!(cube.min_y <= cube.max_y);
            assert!(cube.min_z <= cube.max_z);

            (state == "on", cube)
        })
        .collect()
}

impl Cube {
    pub fn contains(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.min_x
            && x <= self.max_x
            && y >= self.min_y
            && y <= self.max_y
            && z >= self.min_z
            && z <= self.max_z
    }

    pub fn contains_cube(&self, cube2: &Cube) -> bool {
        cube2.min_x >= self.min_x
            && cube2.max_x <= self.max_x
            && cube2.min_y >= self.min_y
            && cube2.max_y <= self.max_y
            && cube2.min_z >= self.min_z
            && cube2.max_z <= self.max_z
    }

    pub fn split_x(&self, cube2: &Cube) -> Vec<Cube> {
        let mut cubes = Vec::new();
        let mut new_min_x = self.min_x;
        let mut new_max_x = self.max_x;
        if cube2.min_x > self.min_x && cube2.min_x <= self.max_x {
            cubes.push(Cube {
                min_x: self.min_x,
                max_x: cube2.min_x - 1,
                min_y: self.min_y,
                max_y: self.max_y,
                min_z: self.min_z,
                max_z: self.max_z,
            });
            new_min_x = cube2.min_x;
        }
        if cube2.max_x >= self.min_x && cube2.max_x < self.max_x {
            cubes.push(Cube {
                min_x: cube2.max_x + 1,
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: self.max_y,
                min_z: self.min_z,
                max_z: self.max_z,
            });
            new_max_x = cube2.max_x;
        }
        cubes.push(Cube {
            min_x: new_min_x,
            max_x: new_max_x,
            min_y: self.min_y,
            max_y: self.max_y,
            min_z: self.min_z,
            max_z: self.max_z,
        });
        cubes
    }

    pub fn split_y(&self, cube2: &Cube) -> Vec<Cube> {
        let mut cubes = Vec::new();
        let mut new_min_y = self.min_y;
        let mut new_max_y = self.max_y;
        if cube2.min_y > self.min_y && cube2.min_y <= self.max_y {
            cubes.push(Cube {
                min_x: self.min_x,
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: cube2.min_y - 1,
                min_z: self.min_z,
                max_z: self.max_z,
            });
            new_min_y = cube2.min_y;
        }
        if cube2.max_y >= self.min_y && cube2.max_y < self.max_y {
            cubes.push(Cube {
                min_x: self.min_x,
                max_x: self.max_x,
                min_y: cube2.max_y + 1,
                max_y: self.max_y,
                min_z: self.min_z,
                max_z: self.max_z,
            });
            new_max_y = cube2.max_y;
        }
        cubes.push(Cube {
            min_x: self.min_x,
            max_x: self.max_x,
            min_y: new_min_y,
            max_y: new_max_y,
            min_z: self.min_z,
            max_z: self.max_z,
        });
        cubes
    }

    pub fn split_z(&self, cube2: &Cube) -> Vec<Cube> {
        let mut cubes = Vec::new();
        let mut new_min_z = self.min_z;
        let mut new_max_z = self.max_z;
        if cube2.min_z > self.min_z && cube2.min_z <= self.max_z {
            cubes.push(Cube {
                min_x: self.min_x,
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: self.max_y,
                min_z: self.min_z,
                max_z: cube2.min_z - 1,
            });
            new_min_z = cube2.min_z;
        }
        if cube2.max_z >= self.min_z && cube2.max_z < self.max_z {
            cubes.push(Cube {
                min_x: self.min_x,
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: self.max_y,
                min_z: cube2.max_z + 1,
                max_z: self.max_z,
            });
            new_max_z = cube2.max_z;
        }
        cubes.push(Cube {
            min_x: self.min_x,
            max_x: self.max_x,
            min_y: self.min_y,
            max_y: self.max_y,
            min_z: new_min_z,
            max_z: new_max_z,
        });
        cubes
    }

    pub fn count_on(&self) -> usize {
        (self.max_x - self.min_x + 1) as usize
            * (self.max_y - self.min_y + 1) as usize
            * (self.max_z - self.min_z + 1) as usize
    }
}
