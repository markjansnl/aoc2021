pub mod input;

pub fn count_fish(input: &str, iterations: usize) -> usize {
    let mut state = input.split(",").fold([0; 9], |mut acc, n| {
        acc[n.parse::<usize>().unwrap()] += 1;
        acc
    });
    for _ in 0..iterations {
        state = [
            state[1],
            state[2],
            state[3],
            state[4],
            state[5],
            state[6],
            state[7] + state[0],
            state[8],
            state[0],
        ];
    }
    state.iter().sum()
}
