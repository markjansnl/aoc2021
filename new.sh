#!/bin/bash

set -e

source session
if [ -z "$1" ]; then
    DAY_LONG=$(date '+%d')
else
    DAY_LONG=$1
fi
DAY_SHORT=$(echo $DAY_LONG | sed 's/^0*//')
FOLDER=aoc$DAY_LONG

cargo new $FOLDER
cd $FOLDER/src
rm -f main.rs
echo 'pub mod input;' > lib.rs
cat > input.rs << EOF
pub const EXAMPLE: &'static str = include_str!("input/example.txt");

pub const USER: &'static str = include_str!("input/user.txt");

EOF
mkdir input
touch input/example.txt
curl "https://adventofcode.com/2021/day/$DAY_SHORT/input" --cookie "session=$SESSION" | perl -pe 'chomp if eof' > input/user.txt
mkdir bin
cat > bin/a.rs << EOF
use $FOLDER::input;

fn f(input: &str) -> usize {
    0
}

fn main() {
    println!("{}", f(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(0, f(input::EXAMPLE));
}
EOF
