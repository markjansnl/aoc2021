use aoc17::{input, Hit, TargetArea, Trajectory};

fn highest_y(input: &str) -> i32 {
    let mut highest_y = 0;
    let target_area: TargetArea = input.into();

    for velocity_y in 0..200 {
        for velocity_x in 0..200 {
            let trajectory = Trajectory::new(velocity_x, velocity_y)
                .map(|(x, y)| (x, y, target_area.hit(x, y)))
                .take_while(|(_, _, hit)| *hit != Hit::Under)
                .collect::<Vec<_>>();
            if trajectory
                .iter()
                .find(|(_, _, hit)| *hit == Hit::Hit)
                .is_some()
            {
                highest_y = highest_y.max(trajectory.iter().map(|(_, y, _)| *y).max().unwrap());
            }
        }
    }

    highest_y
}

fn main() {
    println!("{}", highest_y(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(45, highest_y(input::EXAMPLE));
}
