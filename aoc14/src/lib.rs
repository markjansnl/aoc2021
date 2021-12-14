pub mod input;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pair((char, char));

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        Pair((a, b))
    }
}

impl From<&[char]> for Pair {
    fn from(char_array: &[char]) -> Self {
        let mut chars = char_array.iter().cloned();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        Pair((a, b))
    }
}

pub fn most_minus_least(input: &str, steps: usize) -> usize {
    let mut lines = input.lines();
    let template: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next().unwrap();
    let rules: HashMap<Pair, char> = lines
        .map(|line| {
            let mut split = line.split(" -> ");
            let pair: Pair = split.next().unwrap().into();
            let insert_char = split.next().unwrap().chars().next().unwrap();
            (pair, insert_char)
        })
        .collect();

    let mut pairs: HashMap<Pair, usize> =
        template.windows(2).fold(HashMap::new(), |mut map, pair| {
            map.entry(pair.into())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            map
        });

    let mut char_count: HashMap<char, usize> =
        template.iter().fold(HashMap::new(), |mut map, &c| {
            map.entry(c).and_modify(|count| *count += 1).or_insert(1);
            map
        });

    for _ in 0..steps {
        let mut next_pairs = HashMap::new();
        for (pair, count) in pairs {
            let insert_char = *rules.get(&pair).unwrap();
            add_count(&mut char_count, insert_char, count);
            add_count(&mut next_pairs, Pair((pair.0 .0, insert_char)), count);
            add_count(&mut next_pairs, Pair((insert_char, pair.0 .1)), count);
        }
        pairs = next_pairs;
    }

    let most = char_count.values().max().unwrap();
    let least = char_count.values().min().unwrap();
    most - least
}

fn add_count<T: Eq + Hash>(map: &mut HashMap<T, usize>, key: T, count: usize) {
    map.entry(key)
        .and_modify(|count2| *count2 += count)
        .or_insert(count);
}