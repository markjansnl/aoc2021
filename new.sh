#!/bin/bash

set -e

cargo new $1
cd $1/src
rm -f main.rs
echo 'pub mod input;' > lib.rs
cat > input.rs << EOF
pub const EXAMPLE: &'static str = include_str!("input/example.txt");

pub const USER: &'static str = include_str!("input/user.txt");

EOF
mkdir input
touch input/example.txt
touch input/user.txt
mkdir bin
cat > bin/a.rs << EOF
use $1::input;

fn main() {

}

#[test]
fn test_example() {

}
EOF
