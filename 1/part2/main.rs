use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    // Start position of the dial
    let mut position: i32 = 50;

    // Counter for how many times we land on 0 during any click
    let mut zero_count = 0;

    // Process each line
    for line in input.lines() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse the line
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().expect("Failed to parse number");

        // Count how many times we hit 0 during this rotation
        // Key insight: we hit 0 each time we pass a multiple of 100
        let zeros_hit = match direction {
            'R' => {
                // Right rotation: moving from position to position+distance
                // We cross 0 each time we pass a multiple of 100
                // Must use div_euclid for floor division (not truncation toward zero)
                (position + distance).div_euclid(100) - position.div_euclid(100)
            },
            'L' => {
                // Left rotation: moving from position to position-distance
                // We cross 0 when going backwards through it
                // Must use div_euclid for proper floor division with negatives
                position.div_euclid(100) - (position - distance).div_euclid(100)
            },
            _ => {
                panic!("Unexpected direction: {}", direction);
            }
        };

        zero_count += zeros_hit;

        // Apply the rotation to update position
        match direction {
            'R' => position += distance,
            'L' => position -= distance,
            _ => panic!("Unexpected direction: {}", direction),
        }

        // Normalize position to 0-99 range to prevent overflow
        // We do this AFTER counting zeros, since the counting logic
        // needs to know how many complete wraps we've done
        position = ((position % 100) + 100) % 100;
    }

    println!("Password: {}", zero_count);
}
