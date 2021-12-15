use pathfinding::prelude::{absdiff, astar, Matrix};

pub mod input;

pub fn fastest_path(rows: usize, columns: usize, values: Vec<usize>) -> usize {
    let matrix = Matrix::from_vec(rows, columns, values).unwrap();
    let goal = (columns - 1, rows - 1);

    astar(
        &(0, 0),
        |&p| {
            matrix
                .neighbours(p, false)
                .map(|n| (n, *matrix.get(n).unwrap()))
        },
        |&(x, y)| absdiff(x, columns) + absdiff(y, rows),
        |&p| p == goal,
    )
    .unwrap()
    .1
}
