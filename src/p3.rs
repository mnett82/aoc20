fn count_trees(map: &[&str], dx: usize, dy: usize) -> usize {
    let cols = map.iter().next().unwrap().len();

    map.iter()
        .cloned()
        .step_by(dy)
        .enumerate()
        .filter(|&(row, trees)| trees.as_bytes()[(row * dx) % cols] == '#' as u8)
        .count()
}

pub fn main() {
    let values = include_str!("p3.txt").lines().collect::<Vec<_>>();

    // Part 1:
    println!("Part 1: {}", count_trees(&values, 3, 1));

    // Part 2:
    println!(
        "Part 2: {}",
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(dx, dy)| count_trees(&values, *dx, *dy))
            .fold(1, |acc, x| acc * x)
    );
}
