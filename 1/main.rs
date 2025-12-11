use std::fs;

fn main() {
    // Read the input file
    // In Rust, file operations can fail, so we use expect() to handle errors
    // expect() will panic with our message if the operation fails
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    // Start position of the dial
    let mut position: i32 = 50;

    // Counter for how many times we land on 0
    let mut zero_count = 0;

    // Process each line
    // lines() returns an iterator over the lines of the string
    for line in input.lines() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse the line
        // First character is the direction (L or R)
        // The rest is the distance as a number
        let direction = line.chars().next().unwrap(); // Get first character
        let distance: i32 = line[1..].parse().expect("Failed to parse number");

        // Apply the rotation
        // match is Rust's pattern matching - like a switch statement but more powerful
        match direction {
            'L' => {
                // Rotate left (decrease position)
                position -= distance;
            },
            'R' => {
                // Rotate right (increase position)
                position += distance;
            },
            _ => {
                // This handles any unexpected character
                panic!("Unexpected direction: {}", direction);
            }
        }

        // Handle wrapping using modulo
        // The dial is circular (0-99), so we use modulo 100
        // In Rust, the % operator can give negative results for negative numbers,
        // so we add 100 and mod again to ensure we get a positive result
        position = ((position % 100) + 100) % 100;

        // Check if we landed on 0
        if position == 0 {
            zero_count += 1;
        }

        // Debug output (optional - comment out for clean output)
        // println!("After {}{}: position = {}", direction, distance, position);
    }

    // Print the answer
    println!("Password: {}", zero_count);
}
