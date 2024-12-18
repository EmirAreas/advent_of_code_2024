use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_inputs(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();
        if values.len() >= 2 {
            // Safely parse and push into respective vectors
            if let (Ok(a), Ok(b)) = (values[0].parse::<i32>(), values[1].parse::<i32>()) {
                left.push(a);
                right.push(b);
            }
        }
    }
    Ok((left, right))
}

fn main() {
    let (mut left, mut right) = read_inputs("input.txt").expect("Failed to read inputs");

    // Part 1: Calculate sum of absolute differences
    left.sort_unstable();
    right.sort_unstable();
    let part1: i32 = left.iter()
        .zip(&right)
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("Part 1: {}", part1);

    // Part 2: Weighted sum based on occurrences in 'right'
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    for &value in &right {
        *count_map.entry(value).or_insert(0) += 1;
    }

    let part2: i32 = left.iter()
        .map(|&value| count_map.get(&value).unwrap_or(&0) * value)
        .sum();
    println!("Part 2: {}", part2);
}
