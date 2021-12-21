pub mod input;

pub fn flash(levels: &mut Vec<u8>, i: usize) -> usize {
    let mut flashes = 0;
    let x = (i % 10) as i32;
    let y = (i / 10) as i32;

    if levels[i] > 9 {
        flashes += 1;
        levels[i] = 0;

        for dy in -1..=1i32 {
            let y2 = y + dy;
            if y2 >= 0 && y2 < 10 {
                for dx in -1..=1i32 {
                    let x2 = x + dx;
                    if x2 >= 0 && x2 < 10 {
                        let i2 = (y2 * 10 + x2) as usize;
                        if levels[i2] > 0 {
                            levels[i2] += 1;
                            flashes += flash(levels, i2);
                        }
                    }
                }
            }
        }
    }

    flashes
}
