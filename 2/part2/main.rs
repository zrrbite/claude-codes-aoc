use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    // Parse the ranges - they're comma-separated on one line
    let ranges_str = input.trim();

    // Total sum of all invalid IDs
    let mut total: u64 = 0;

    // Split by comma to get individual ranges
    for range in ranges_str.split(',') {
        // Each range is "start-end"
        let parts: Vec<&str> = range.split('-').collect();

        if parts.len() != 2 {
            continue; // Skip malformed ranges
        }

        // Parse start and end of range
        let start: u64 = parts[0].parse().expect("Failed to parse start");
        let end: u64 = parts[1].parse().expect("Failed to parse end");

        // Check each number in the range
        for num in start..=end {
            if is_invalid_id(num) {
                total += num;
            }
        }
    }

    println!("Sum of invalid IDs: {}", total);
}

/// Check if a number is an "invalid ID" - a pattern repeated at least twice
/// Examples: 111 (1 three times), 123123 (123 twice), 12341234 (1234 twice)
fn is_invalid_id(num: u64) -> bool {
    // Convert number to string to analyze its digits
    let s = num.to_string();
    let len = s.len();

    // Try all possible pattern lengths (from 1 up to half the length)
    // We need at least 2 repetitions, so pattern_len <= len/2
    for pattern_len in 1..=len / 2 {
        // Only check if the pattern length divides evenly into total length
        if len % pattern_len == 0 {
            // Extract the pattern (first pattern_len characters)
            let pattern = &s[..pattern_len];

            // Check if the entire string is this pattern repeated
            if is_repeated_pattern(&s, pattern) {
                return true;
            }
        }
    }

    false
}

/// Check if a string consists entirely of a pattern repeated
fn is_repeated_pattern(s: &str, pattern: &str) -> bool {
    // Split the string into chunks of pattern length
    // and check if each chunk equals the pattern
    s.chars()
        .collect::<Vec<char>>()
        .chunks(pattern.len())
        .all(|chunk| {
            chunk.iter().collect::<String>() == pattern
        })
}
