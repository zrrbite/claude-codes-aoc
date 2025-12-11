# Day 1 - Part 1: Safe Dial

## Problem Summary
You have a circular dial with positions 0-99. Starting at position 50, you need to follow a sequence of rotations:
- `L` = rotate left (toward lower numbers)
- `R` = rotate right (toward higher numbers)
- The dial wraps around (0 connects to 99)

**Goal**: Count how many times the dial ends on position 0 after each rotation.

## Solution Approach

1. **Parse each instruction** - Extract direction (L/R) and distance
2. **Apply rotation** - Update position based on direction
3. **Handle wrapping** - Use modulo arithmetic for circular behavior
4. **Check for zero** - Count when final position equals 0

## Key Rust Concepts

### 1. Variables and Mutability
```rust
let mut position: i32 = 50;
```
- Variables are **immutable by default** in Rust
- Use `mut` keyword to make them mutable
- `i32` is a 32-bit signed integer type (explicit typing)

### 2. Error Handling with `expect()`
```rust
let input = fs::read_to_string("input.txt")
    .expect("Failed to read input file");
```
- File operations return `Result<T, E>` (success or error)
- `expect()` unwraps the value or panics with a message
- Forces explicit error handling - you can't ignore errors in Rust!

### 3. Pattern Matching
```rust
match direction {
    'L' => { position -= distance; },
    'R' => { position += distance; },
    _ => { panic!("Unexpected direction: {}", direction); }
}
```
- `match` is like a switch statement but **exhaustive**
- Must handle all possible cases
- `_` is a catch-all pattern for "anything else"
- Much safer than traditional switch statements

### 4. String Manipulation
```rust
let direction = line.chars().next().unwrap();
let distance: i32 = line[1..].parse().expect("...");
```
- `chars()` returns an iterator over Unicode characters
- `[1..]` is a **slice** - takes from index 1 to end
- `parse()` attempts type conversion based on the target type
- Type inference determines what `parse()` should produce

### 5. Modular Arithmetic with Wrapping
```rust
position = ((position % 100) + 100) % 100;
```
**Why the double modulo?**
- In Rust, `%` can return negative values: `-18 % 100 = -18`
- Adding 100 ensures positive: `(-18 + 100) % 100 = 82`
- This handles wrapping in both directions correctly

**Examples:**
- `150 % 100 = 50` → `(50 + 100) % 100 = 50` ✓
- `-18 % 100 = -18` → `(-18 + 100) % 100 = 82` ✓

## Code Flow

1. Read input file
2. Initialize position to 50, counter to 0
3. For each line:
   - Parse direction and distance
   - Apply rotation (add for R, subtract for L)
   - Normalize position to 0-99 range
   - If position is 0, increment counter
4. Print the total count

## Answer
**Password: 1154**

The dial landed on position 0 exactly 1,154 times across all rotations.
