use aoc22::{input, parse_input, Cube};

fn count_cubes_on(input: &str) -> usize {
    let mut cubes = Vec::new();
    for (index, (state, cube)) in parse_input(input).into_iter().enumerate() {
        cubes = cubes
            .into_iter()
            .map(|cube2: Cube| cube2.split_x(&cube))
            .flatten()
            .collect();
        cubes = cubes
            .into_iter()
            .map(|cube2: Cube| cube2.split_y(&cube))
            .flatten()
            .collect();
        cubes = cubes
            .into_iter()
            .map(|cube2: Cube| cube2.split_z(&cube))
            .flatten()
            .collect();

        cubes.retain(|cube2| !cube.contains_cube(cube2));

        if state {
            cubes.push(cube);
        }
        println!("{}", index);
    }
    cubes.iter().map(Cube::count_on).sum()
}

fn main() {
    println!("{}", count_cubes_on(input::USER));
}

#[test]
fn test_example1() {
    assert_eq!(39, count_cubes_on(input::EXAMPLE1));
}

#[test]
fn test_example3() {
    assert_eq!(2758514936282235, count_cubes_on(input::EXAMPLE3));
}
