use std::collections::{HashMap, HashSet};

pub mod input;

pub enum Axis {
    X,
    Y,
    Z,
}

pub type Beacon = (i16, i16, i16);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scan {
    index: u8,
    beacons: HashSet<Beacon>,
}

impl Scan {
    pub fn new(index: u8) -> Self {
        Scan {
            index,
            beacons: HashSet::new(),
        }
    }

    pub fn add_beacon(&mut self, beacon: Beacon) {
        self.beacons.insert(beacon);
    }

    pub fn rotate(&self, axis: Axis) -> Scan {
        Scan {
            index: self.index,
            beacons: self
                .beacons
                .iter()
                .map(|&(x, y, z)| match axis {
                    Axis::X => (x, -z, y),
                    Axis::Y => (-z, y, x),
                    Axis::Z => (-y, x, z),
                })
                .collect(),
        }
    }

    pub fn overlaps(&self, other: &Scan, min_beacons_overlap: usize) -> Option<Beacon> {
        let mut deltas: HashMap<Beacon, usize> = HashMap::new();
        let mut result = None;

        for (x1, y1, z1) in &self.beacons {
            for (x2, y2, z2) in &other.beacons {
                let delta = (x2 - x1, y2 - y1, z2 - z1);
                deltas
                    .entry(delta)
                    .and_modify(|count| {
                        *count += 1;
                        if *count == min_beacons_overlap {
                            result = Some(delta);
                        }
                    })
                    .or_insert(1);
                if result.is_some() {
                    return result;
                }
            }
        }
        result
    }

    pub fn faces_iter(&self) -> ScanFacesIterator {
        ScanFacesIterator {
            initial: self.clone(),
            previous: self.clone(),
            index: 0,
        }
    }

    pub fn all_directions_iter(&self) -> impl Iterator<Item = Scan> {
        self.faces_iter().flat_map(|face| {
            let rotate1 = face.rotate(Axis::Z);
            let rotate2 = rotate1.rotate(Axis::Z);
            let rotate3 = rotate2.rotate(Axis::Z);
            [face, rotate1, rotate2, rotate3]
        })
    }

    pub fn count_beacons(&self) -> usize {
        self.beacons.len()
    }

    pub fn beacons(&self) -> HashSet<Beacon> {
        self.beacons.clone()
    }
}

pub fn input_to_scans(input: &str) -> Vec<Scan> {
    let mut index = 0;
    input
        .lines()
        .filter(|line| line.len() > 0)
        .fold(Vec::new(), |mut scans, line| {
            if line[0..3] == *"---" {
                scans.push(Scan::new(index));
                index += 1;
            } else {
                let mut coord_iter = line.split(",").map(|coord| coord.parse::<i16>().unwrap());
                let x = coord_iter.next().unwrap();
                let y = coord_iter.next().unwrap();
                let z = coord_iter.next().unwrap();
                scans.last_mut().unwrap().add_beacon((x, y, z));
            }
            scans
        })
}

pub struct ScanFacesIterator {
    initial: Scan,
    previous: Scan,
    index: u8,
}

impl Iterator for ScanFacesIterator {
    type Item = Scan;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => Some(self.initial.clone()),
            2 | 3 | 4 => {
                self.previous = self.previous.rotate(Axis::Y);
                Some(self.previous.clone())
            }
            5 => {
                self.previous = self.initial.rotate(Axis::X);
                Some(self.previous.clone())
            }
            6 => {
                self.previous = self.previous.rotate(Axis::X).rotate(Axis::X);
                Some(self.previous.clone())
            }
            _ => None,
        }
    }
}
