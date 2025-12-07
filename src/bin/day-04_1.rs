use std::fs;
use std::io;

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-04");

struct Diagram {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<u8>>,
}

#[rustfmt::skip]
const DIRS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1), /*----*/ ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn main() {
    let diagram = parse_input(INPUT_PATH).unwrap(); // Never fails

    dbg!(count_accessible(&diagram));
}

fn count_accessible(diagram: &Diagram) -> usize {
    let mut accessible = 0;

    for row in 0..diagram.rows {
        for col in 0..diagram.cols {
            if diagram.grid[row][col] != b'@' {
                continue;
            }

            let adjacent = DIRS
                .iter()
                .filter(|(dx, dy)| {
                    let r = (row as isize + dx) as usize;
                    let c = (col as isize + dy) as usize;

                    r < diagram.rows && c < diagram.cols && diagram.grid[r][c] == b'@'
                })
                .count();

            if adjacent < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

fn parse_input(path: &str) -> io::Result<Diagram> {
    let input = fs::read_to_string(path)?;

    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let rows = grid.len();
    let cols = grid.first().map_or(0, |row| row.len());

    Ok(Diagram { rows, cols, grid })
}
