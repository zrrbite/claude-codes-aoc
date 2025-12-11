# Day 1 - Part 2: Safe Dial with Click Counting

## Problem Summary
Same dial setup as Part 1, but now we count **every click** that lands on 0, not just the final position.

**Key difference**: A single rotation like `R1000` from position 50 would cross 0 **ten times** during the rotation!

## Solution Approach

Instead of just checking if we end on 0, we need to count how many times we **pass through** 0 during each rotation.

**Mathematical insight**: Count how many multiples of 100 we cross during the rotation.

### For RIGHT rotation by `distance` from `position`:
```
zeros_crossed = (position + distance) / 100 - position / 100
```

### For LEFT rotation by `distance` from `position`:
```
zeros_crossed = position / 100 - (position - distance) / 100
```

## Key Rust Concepts

### 1. Integer Division for Counting
```rust
(position + distance) / 100 - position / 100
```
- The `/` operator performs **integer division** for integer types
- Automatically truncates to floor for positive numbers
- Perfect for counting complete cycles

**Example**: `R150` from position 60
- Start: `60`
- End: `60 + 150 = 210`
- Zeros crossed: `210/100 - 60/100 = 2 - 0 = 2`
- We crossed 0 at positions 100 and 200 ✓

### 2. div_euclid() - The Critical Difference!
```rust
position.div_euclid(100) - (position - distance).div_euclid(100)
```
**Why not use `/`?**
- Rust's `/` operator **truncates toward zero**: `-18 / 100 = 0`
- We need **floor division**: `-18 ÷ 100 = -1` (rounds down)
- `div_euclid()` performs Euclidean division (floor division)

**The bug this fixes:**
```rust
// WRONG (using /):
50 / 100 - (50 - 68) / 100
= 0 - (-18) / 100
= 0 - 0 = 0  ✗ (misses the zero crossing!)

// RIGHT (using div_euclid):
50.div_euclid(100) - (50 - 68).div_euclid(100)
= 0 - (-18).div_euclid(100)
= 0 - (-1) = 1  ✓ (correctly counts crossing 0)
```

**Example**: `L68` from position 50
- Start: `50`
- End: `50 - 68 = -18`
- Zeros crossed: `50.div_euclid(100) - (-18).div_euclid(100) = 0 - (-1) = 1`
- We crossed 0 once going from 50 backward to -18 (which is position 82) ✓

### 3. Why We Normalize AFTER Counting
```rust
// Count zeros FIRST (needs actual accumulated value)
let zeros_hit = (position + distance) / 100 - position / 100;

// THEN normalize position (for display/next iteration)
position = ((position % 100) + 100) % 100;
```

**Critical distinction**:
- The counting logic needs the **actual position value** (can be > 99 or < 0)
- We keep position as accumulated clicks, not normalized 0-99
- Only normalize at the end for display purposes

**Why this matters**:
```rust
// Starting at 50, do R60 then R60
position = 50;

// First R60:
zeros = (50 + 60)/100 - 50/100 = 110/100 - 0 = 1  ✓
position = 110

// If we normalized here: position = 10
// Second R60:
// WRONG: (10 + 60)/100 - 10/100 = 70/100 - 0 = 0  ✗
// RIGHT: (110 + 60)/100 - 110/100 = 170/100 - 1 = 1  ✓
```

### 4. Match Expressions as Values
```rust
let zeros_hit = match direction {
    'R' => (position + distance) / 100 - position / 100,
    'L' => position / 100 - (position - distance) / 100,
    _ => panic!("Unexpected direction: {}", direction),
};
```
- `match` is an **expression** in Rust, not just a statement
- It returns a value from the matched arm
- All arms must return the same type

## How the Math Works

### Example Walkthrough
Starting position: 50

**L68** (left by 68):
- Virtual position: `50 - 68 = -18`
- Zeros: `50/100 - (-18)/100 = 0 - (-1) = 1` ✓
- Actual position on dial: 82

**R48** from 82:
- Virtual position: `82 + 48 = 130`
- Zeros: `130/100 - 82/100 = 1 - 0 = 1` ✓
- Actual position on dial: 30

**R1000** from 50 (extreme example):
- Virtual position: `50 + 1000 = 1050`
- Zeros: `1050/100 - 50/100 = 10 - 0 = 10` ✓
- We cross 0 at: 100, 200, 300, ..., 1000 (ten times!)

## Code Flow

1. Read input file
2. Initialize position to 50, counter to 0
3. For each line:
   - Parse direction and distance
   - **Count zeros crossed** using division formula
   - Add to total count
   - Apply rotation to position
   - Normalize position to 0-99 for next iteration
4. Print the total count

## Answer
**Password: 6819**

The dial pointed at 0 exactly 6,819 times across all rotations (including both during rotations and at end positions).

## Learning Insights

1. **div_euclid() vs /** - Know the difference! Floor division vs truncation toward zero
2. **Integer division is powerful** for counting discrete events
3. **Don't normalize too early** - keep track of absolute position for counting
4. **Negative numbers** require careful handling in division
5. **Match expressions** make conditional logic clean and type-safe

## Common Pitfall

**Beware the `/` operator with negative numbers!**

In many mathematical formulas, you assume floor division. But Rust's `/` truncates toward zero, which gives different results for negative dividends. Always consider whether you need `div_euclid()` instead!
