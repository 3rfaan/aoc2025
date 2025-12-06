use std::{fs, io};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-02");

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn main() {
    let ranges = parse_input(INPUT_PATH).unwrap(); // Never fails

    // Sum invalid IDs from all ranges
    let invalid_sum: u64 = ranges.iter().map(sum_invalid_ids).sum();

    dbg!(invalid_sum);
}

fn sum_invalid_ids(range: &Range) -> u64 {
    (range.start..=range.end)
        .filter(|&id| check_invalid(id))
        .sum()
}

/// n: integer, d: number of digits
fn check_invalid(n: u64) -> bool {
    let digits = count_digits(n);

    if !digits.is_multiple_of(2) {
        return false;
    }

    let divisor = 10u64.pow(digits / 2);

    let lo = n % divisor;
    let hi = n / divisor;

    hi == lo
}

#[inline]
fn count_digits(n: u64) -> u32 {
    if n < 10 { 1 } else { n.ilog10() + 1 }
}

fn parse_input(path: &str) -> io::Result<Vec<Range>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .split_terminator(',')
        .filter_map(|range| {
            let (start, end) = range.trim().split_once('-')?;

            Some(Range {
                start: start.parse().ok()?,
                end: end.parse().ok()?,
            })
        })
        .collect())
}
