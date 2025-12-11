fn main() {
    // Test the division behavior
    println!("Testing division:");
    println!("-18 / 100 = {}", -18 / 100);  // Truncates toward zero
    println!("50 / 100 = {}", 50 / 100);
    
    // Example from puzzle: L68 from position 50
    let position = 50;
    let distance = 68;
    
    // My formula
    let zeros = position / 100 - (position - distance) / 100;
    println!("\nL68 from 50:");
    println!("Formula result: {}", zeros);
    println!("Expected: 1 (should cross 0 once)");
}
