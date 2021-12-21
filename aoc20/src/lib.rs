use std::collections::HashSet;

pub mod input;

#[derive(Default)]
struct Boundries {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

pub fn count_lit_pixels(input: &str, enhancement_times: usize) -> usize {
    let mut boundries = Boundries::default();
    let mut infinite_pixel = 0;

    let mut lines = input.lines();
    let algorithm = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| b == b'#')
        .collect::<Vec<bool>>();
    lines.next().unwrap();

    let mut pixels = HashSet::new();
    for (y, line) in lines.enumerate() {
        boundries.max_y += 1;
        for (x, b) in line.bytes().enumerate() {
            if y == 0 {
                boundries.max_x += 1;
            }
            if b == b'#' {
                pixels.insert((x as i32, y as i32));
            }
        }
    }

    for _ in 0..enhancement_times {
        let mut pixels2 = HashSet::new();
        for y in boundries.min_y - 1..=boundries.max_y + 1 {
            for x in boundries.min_x - 1..=boundries.max_x + 1 {
                let index = (0..9).fold(0, |acc, b| {
                    let x2 = x - 1 + (b % 3);
                    let y2 = y - 1 + (b / 3);
                    if x2 >= boundries.min_x
                        && x2 <= boundries.max_x
                        && y2 >= boundries.min_y
                        && y2 <= boundries.max_y
                    {
                        (acc << 1) + if pixels.contains(&(x2, y2)) { 1 } else { 0 }
                    } else {
                        (acc << 1) + infinite_pixel
                    }
                });
                if *algorithm.get(index).unwrap() {
                    pixels2.insert((x, y));
                }
            }
        }
        pixels = pixels2;

        boundries = pixels.iter().fold(boundries, |acc, &(x, y)| Boundries {
            min_x: acc.min_x.min(x),
            min_y: acc.min_y.min(y),
            max_x: acc.max_x.max(x),
            max_y: acc.max_y.max(y),
        });

        infinite_pixel = if *algorithm.get(infinite_pixel * 511).unwrap() {
            1
        } else {
            0
        };
    }

    pixels.len()
}
