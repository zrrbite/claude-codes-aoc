use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    // Parse the ranges - they're comma-separated on one line
    // Remove any whitespace/newlines and split by comma
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

/// Check if a number is an "invalid ID" - a pattern repeated twice
/// Examples: 11 (1 repeated), 6464 (64 repeated), 123123 (123 repeated)
fn is_invalid_id(num: u64) -> bool {
    // Convert number to string to analyze its digits
    let s = num.to_string();

    // Length must be even to be split in half
    if s.len() % 2 != 0 {
        return false;
    }

    // Check for leading zeros (though u64 won't have them when converted)
    // This is already handled by using u64

    // Split string in half
    let mid = s.len() / 2;
    let first_half = &s[..mid];   // First half of digits
    let second_half = &s[mid..];  // Second half of digits

    // Check if both halves are identical
    first_half == second_half
}
