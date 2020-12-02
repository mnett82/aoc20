//! https://adventofcode.com/2020/day/1.

use tinyset::Set64;

fn find_pair(values: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut seen = Set64::new();

    values
        .iter()
        .cloned()
        .filter_map(|x| {
            seen.insert(x);

            if seen.contains(&(sum - x)) {
                Some((x, sum - x))
            } else {
                None
            }
        })
        .next()
}

fn find_triple(values: &[i32], sum: i32) -> Option<(i32, i32, i32)> {
    values
        .iter()
        .cloned()
        .filter_map(|value| find_pair(values, sum - value).map(|(x, y)| (value, x, y)))
        .next()
}

fn main() {
    let values = include_str!("p1.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // Part 1:
    let (x, y) = find_pair(&values, 2020).expect("Found no solution to part 1.");
    println!("Part 1: {}, {} --> {}", x, y, x * y);

    // Part 2:
    let (x, y, z) = find_triple(&values, 2020).expect("Found no solution to part 2.");
    println!("Part 2: {}, {}, {} --> {}", x, y, z, x * y * z);
}
