use aoc17::{input, Hit, TargetArea, Trajectory};

fn highest_y(input: &str) -> usize {
    let mut count = 0;
    let target_area: TargetArea = input.into();

    for velocity_y in -1000..1000 {
        for velocity_x in -1000..1000 {
            let trajectory = Trajectory::new(velocity_x, velocity_y)
                .map(|(x, y)| (x, y, target_area.hit(x, y)))
                .take_while(|(_, _, hit)| *hit != Hit::Under)
                .collect::<Vec<_>>();
            if trajectory
                .iter()
                .find(|(_, _, hit)| *hit == Hit::Hit)
                .is_some()
            {
                count += 1
            }
        }
    }

    count
}

fn main() {
    println!("{}", highest_y(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(112, highest_y(input::EXAMPLE));
}
