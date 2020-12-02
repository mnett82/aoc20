//! https://adventofcode.com/2020/day/2.

use scan_fmt::scan_fmt;

struct Entry {
    i: u8,
    j: u8,
    sym: char,
    text: String,
}

impl Entry {
    fn parse(input: &str) -> Self {
        let (i, j, sym, text) = scan_fmt!(input, "{d}-{d} {}: {}", u8, u8, char, String)
            .expect(&format!("Failed to parse '{}'", input));

        Self { i, j, sym, text }
    }

    fn has_valid_occurrence_count(&self) -> bool {
        let mut count = 0;
        for c in self.text.chars() {
            if c == self.sym {
                count += 1;
            }
            if count > self.j {
                return false;
            }
        }
        self.i <= count
    }

    fn has_valid_symbol_at_index(&self) -> bool {
        let bytes = self.text.as_bytes();
        let has_sym_at = |pos: usize| pos < bytes.len() && bytes[pos] == self.sym as u8;

        has_sym_at((self.i - 1) as usize) ^ has_sym_at((self.j - 1) as usize)
    }
}

pub fn main() {
    let entries = include_str!("p2.txt")
        .lines()
        .map(|line| Entry::parse(line))
        .collect::<Vec<_>>();

    // Part 1:
    println!(
        "Part 1: {}",
        entries
            .iter()
            .filter(|&e| e.has_valid_occurrence_count())
            .count()
    );

    // Part 2:
    println!(
        "Part 2: {}",
        entries
            .iter()
            .filter(|&e| e.has_valid_symbol_at_index())
            .count()
    );
}
