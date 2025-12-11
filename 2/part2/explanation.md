# Day 2 Part 2: Gift Shop - Repeated Patterns (At Least Twice)

## Problem Summary
Find all "invalid" product IDs where the number is a pattern repeated **at least twice** (not just exactly twice).

**Examples:**
- `111` = "1" × 3 ✓
- `123123` = "123" × 2 ✓
- `123123123` = "123" × 3 ✓
- `1212121212` = "12" × 5 ✓
- `12341234` = "1234" × 2 ✓

## Solution Approach

**Algorithm:**
1. For each number, try all possible pattern lengths (1 to length/2)
2. Only check pattern lengths that divide evenly into total length
3. Check if the entire string is that pattern repeated

**Why length/2?** If pattern is longer than half, it can't repeat at least twice!

**Example: `123123` (length 6)**
- Try pattern_len 1: "1" repeated? No
- Try pattern_len 2: "12" repeated? No
- Try pattern_len 3: "123" repeated? Yes! ✓

## Key Rust Concepts & Tricks

### 1. Iterator's `all()` Method - Universal Quantification
```rust
s.chars()
    .collect::<Vec<char>>()
    .chunks(pattern.len())
    .all(|chunk| {
        chunk.iter().collect::<String>() == pattern
    })
```

**The Rust Trick:**
- `all()` returns `true` if **every element** satisfies the predicate
- Short-circuits on first `false` (efficient!)
- Think of it as "for all" in mathematics

**Common iterator testing methods:**
```rust
// all() - every element must satisfy condition
[2, 4, 6].iter().all(|&x| x % 2 == 0)  // true

// any() - at least one element must satisfy
[1, 3, 4].iter().any(|&x| x % 2 == 0)  // true (4 is even)

// find() - get first element that satisfies
[1, 3, 4].iter().find(|&&x| x > 2)  // Some(&3)

// position() - get index of first match
[1, 3, 4].iter().position(|&x| x > 2)  // Some(1)
```

**Pattern:** Use these instead of manual loops for cleaner, more expressive code!

### 2. The `chunks()` Method - Slice Subdivision
```rust
vec.chunks(n)  // Split into non-overlapping chunks of size n
```

**The Rust Trick:**
- Divides a slice into chunks of at most `n` elements
- Returns an iterator over slices
- Last chunk may be smaller if length doesn't divide evenly

**Examples:**
```rust
let v = vec![1, 2, 3, 4, 5];

// chunks(2) → [[1,2], [3,4], [5]]
for chunk in v.chunks(2) {
    println!("{:?}", chunk);
}

// chunks_exact(2) → [[1,2], [3,4]] (drops incomplete chunk)
for chunk in v.chunks_exact(2) {
    println!("{:?}", chunk);
}
```

**Related methods:**
```rust
chunks(n)        // Non-overlapping, allows incomplete last chunk
chunks_exact(n)  // Non-overlapping, drops incomplete last chunk
windows(n)       // Overlapping windows of size n
```

### 3. Turbofish Syntax `::<T>` - Explicit Type Parameters
```rust
chunk.iter().collect::<String>()
```

**The Rust Trick:**
- `::<Type>` explicitly specifies generic type parameters
- Called "turbofish" because `::<>` looks like a fish
- Use when compiler can't infer the type

**Common patterns:**
```rust
// Can't infer what collection to create
let nums = input.split(',').collect::<Vec<&str>>();

// Can't infer what type to parse to
let n = "42".parse::<i32>().unwrap();

// Alternative: type annotation instead
let nums: Vec<&str> = input.split(',').collect();
let n: i32 = "42".parse().unwrap();
```

**When you need turbofish:**
- Multiple possible return types
- Generic functions without context
- Disambiguation in complex expressions

### 4. Closures and Captures - Anonymous Functions
```rust
.all(|chunk| {
    chunk.iter().collect::<String>() == pattern
})
```

**The Rust Trick:**
- `|args| expr` creates a closure (anonymous function)
- Closures can **capture** variables from surrounding scope
- `pattern` is captured from the outer function

**Closure types in Rust:**

| Type | Captures | Example |
|------|----------|---------|
| `Fn` | By reference | `\|x\| x + y` (reads `y`) |
| `FnMut` | By mutable reference | `\|x\| { sum += x; }` (modifies `sum`) |
| `FnOnce` | By value | `\|x\| take_ownership(y)` (consumes `y`) |

**Common closure patterns:**
```rust
// Simple expression
nums.iter().filter(|&x| x > 10)

// Block with multiple statements
nums.iter().map(|x| {
    let doubled = x * 2;
    doubled + 1
})

// Capturing outer variables
let threshold = 10;
nums.iter().filter(|&x| x > threshold)  // Captures threshold
```

**Pro tip:** Closures are more efficient than function pointers - they can be inlined!

### 5. The `collect()` Flexibility
```rust
chunk.iter().collect::<String>()  // Collect chars into String
```

**The Rust Trick:**
- `collect()` is **generic** over the target collection
- Can create `Vec`, `String`, `HashMap`, etc.
- The turbofish tells it what to build

**Examples:**
```rust
let chars = vec!['a', 'b', 'c'];

// Collect to String
let s: String = chars.iter().collect();  // "abc"

// Collect to Vec
let v: Vec<char> = chars.iter().copied().collect();

// Collect to Result/Option (very powerful!)
let nums = vec!["1", "2", "not a number"];
let parsed: Result<Vec<i32>, _> = nums.iter()
    .map(|s| s.parse::<i32>())
    .collect();  // Fails because "not a number" doesn't parse
```

**The magic:** `collect()` can even collect `Iterator<Result<T>>` into `Result<Vec<T>>`!

### 6. Ranges in For Loops - Iterator Sugar
```rust
for pattern_len in 1..=len / 2 {
    // ...
}
```

**Range types:**
```rust
1..5      // 1, 2, 3, 4 (exclusive end)
1..=5     // 1, 2, 3, 4, 5 (inclusive end)
..5       // up to 5 (start at 0)
1..       // from 1 to infinity (careful!)
..        // full range (all elements)
```

**Pro tip:** Ranges are lazy iterators - they don't allocate!

## Rust Idioms Demonstrated

### Pattern: Early Return for Invalid Cases
```rust
if len % pattern_len == 0 {
    // Only check if divisible
}
```
Skip unnecessary work - if pattern length doesn't divide evenly, it can't be a perfect repetition.

### Pattern: Helper Functions for Clarity
```rust
fn is_repeated_pattern(s: &str, pattern: &str) -> bool {
    // ...
}
```
Extract complex logic into named functions - improves readability and testability.

### Pattern: Iterator Chains for Data Processing
```rust
s.chars()
    .collect::<Vec<char>>()
    .chunks(pattern.len())
    .all(|chunk| ...)
```
Transform data step-by-step with method chaining - reads like a pipeline.

## Alternative Approaches

### Approach 1: String Repetition (More Efficient!)
```rust
fn is_repeated_pattern(s: &str, pattern: &str) -> bool {
    let repetitions = s.len() / pattern.len();
    s == pattern.repeat(repetitions)
}
```
**Rust trick:** `str::repeat()` creates a new String with pattern repeated N times.

### Approach 2: Without Collect (Zero Allocation!)
```rust
fn is_repeated_pattern(s: &str, pattern: &str) -> bool {
    s.as_bytes()
        .chunks(pattern.len())
        .all(|chunk| chunk == pattern.as_bytes())
}
```
**Rust trick:** Work with bytes directly, avoid String allocation.

### Approach 3: Manual Loop (Most Explicit)
```rust
fn is_repeated_pattern(s: &str, pattern: &str) -> bool {
    let pattern_len = pattern.len();
    let bytes = s.as_bytes();
    let pat_bytes = pattern.as_bytes();

    for i in (0..bytes.len()).step_by(pattern_len) {
        let chunk = &bytes[i..i + pattern_len.min(bytes.len() - i)];
        if chunk != pat_bytes {
            return false;
        }
    }
    true
}
```

**Which to choose?**
- Approach 1: Clearest intent, good for small patterns
- Approach 2: Best performance for large data
- Approach 3: When you need fine control

## Performance Considerations

1. **`collect::<Vec<_>>()` allocates** - Necessary here for `chunks()`, but be aware
2. **`all()` short-circuits** - Stops at first false, very efficient
3. **Pattern length optimization** - We only try divisors of the total length

## Common Gotchas

### Gotcha 1: chars() vs bytes()
```rust
"hello".chars()  // Unicode characters (correct for text)
"hello".as_bytes()  // UTF-8 bytes (faster for ASCII)
```
For digit patterns, either works, but `bytes()` is slightly faster.

### Gotcha 2: chunks() Last Chunk
```rust
[1,2,3,4,5].chunks(2)  // [[1,2], [3,4], [5]] - last is size 1!
```
In our case, we only check when `len % pattern_len == 0`, so last chunk is always full.

## Answer
**Sum of invalid IDs: 28,915,664,389**

## Learning Insights

1. **`all()` and `any()`** replace many manual loop patterns
2. **`chunks()`** is perfect for dividing data into equal-sized pieces
3. **Turbofish `::<T>`** disambiguates generic types
4. **Closures capture variables** from surrounding scope efficiently
5. **`collect()` is magical** - it can build many different types
6. **Helper functions** improve code organization and testability
7. **Multiple approaches** exist - choose based on clarity vs performance needs
