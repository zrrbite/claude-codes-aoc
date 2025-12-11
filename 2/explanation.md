# Day 2: Gift Shop - Invalid Product IDs

## Problem Summary
Find all "invalid" product IDs within given ranges. An invalid ID is a number that's just a pattern repeated twice (like `123123`, `6464`, `99`). Sum all invalid IDs found.

**Rules:**
- Invalid ID = sequence of digits repeated exactly twice
- No leading zeros (though this is handled naturally by using integers)
- Examples: `11`, `99`, `1010`, `123123`, `446446`

## Solution Approach

1. **Parse the input** - Extract comma-separated ranges (`start-end` format)
2. **Check each number** in each range
3. **Detect pattern** - Is the number's string representation a repeated sequence?
4. **Sum all matches**

**Pattern detection logic:**
```
Number: 123123
String: "123123"
Length: 6 (even ✓)
First half: "123"
Second half: "123"
Match? YES → Invalid ID!
```

## Key Rust Concepts & Tricks

### 1. String Slicing - Zero-Cost Abstractions
```rust
let s = "123123";
let mid = s.len() / 2;
let first_half = &s[..mid];   // "123"
let second_half = &s[mid..];  // "123"
```

**The Rust Trick:**
- Slices are **borrowed views** - no copying, no allocation!
- `&s[..mid]` compiles to just a pointer + length
- Compare this to other languages that might copy the substring

**Range syntax patterns:**
```rust
&s[..]      // Full slice (all elements)
&s[..n]     // From start up to (not including) n
&s[n..]     // From n to end
&s[a..b]    // From a up to (not including) b
&s[a..=b]   // From a up to and including b
```

**Common gotcha:**
```rust
let s = "hello";
let c = s[0];  // ❌ ERROR! Can't index strings directly
let c = &s[0..1];  // ✓ Works, but returns &str, not char
let c = s.chars().next();  // ✓ Better for getting first char
```

Why? Rust strings are UTF-8, so indexing by byte could split a multi-byte character!

### 2. Iterator Patterns - Lazy Evaluation Power
```rust
for range in ranges_str.split(',') {
    // Process each range
}
```

**The Rust Trick:**
- `split()` is **lazy** - it doesn't do work until you consume it
- Returns string slices (`&str`) - no allocations!
- Can chain iterator methods for powerful transformations

**Common patterns:**
```rust
// Collecting into a Vec (allocates)
let parts: Vec<&str> = "10-20".split('-').collect();

// Iterating directly (no allocation, more efficient!)
for part in "10-20".split('-') {
    println!("{}", part);
}

// Chaining operations
let sum: i32 = "1,2,3,4"
    .split(',')
    .filter_map(|s| s.parse::<i32>().ok())
    .sum();
```

**Pro tip:** Avoid `collect()` unless you need the Vec! Iterators are more efficient:
```rust
// Less efficient (allocates Vec)
let parts = input.split(',').collect::<Vec<_>>();
for part in parts { }

// More efficient (no allocation)
for part in input.split(',') { }
```

### 3. Range Iteration with `..=`
```rust
for num in start..=end {
    // Process each number
}
```
- `start..end` - Exclusive end (doesn't include `end`)
- `start..=end` - **Inclusive end** (includes `end`)
- This is crucial! For range `11-22`, we need to check both 11 AND 22

**Example:**
```rust
for i in 1..5   // 1, 2, 3, 4
for i in 1..=5  // 1, 2, 3, 4, 5
```

### 4. Implicit Returns - Expression-Based Language
```rust
fn is_invalid_id(num: u64) -> bool {
    // ...
    first_half == second_half  // No semicolon = return value!
}
```

**The Rust Trick:**
- Rust is **expression-based** - almost everything returns a value
- Omit semicolon on last line to return that value
- Adding `;` makes it a statement (returns `()` unit type)

**This is idiomatic:**
```rust
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }  // Clean!
}
```

**This is not idiomatic:**
```rust
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}
```

**When to use explicit `return`:**
- Early returns (guard clauses)
- Breaking out of loops/branches
- When it improves readability

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;  // Early return for error case
    }
    Some(a / b)  // Implicit return for success case
}
```

### 5. The `u64` Type
```rust
let mut total: u64 = 0;
```
- **64-bit unsigned integer** (0 to 18,446,744,073,709,551,615)
- We use `u64` instead of `i32` because:
  - Product IDs can be very large (e.g., `5353503564`)
  - The sum can be even larger (21+ billion)
  - No negative numbers needed

**Integer type choices in Rust:**
- `i32` - Most common, -2.1B to 2.1B
- `i64` - Large signed integers
- `u32` - Unsigned 32-bit (0 to 4.2B)
- `u64` - Unsigned 64-bit (0 to 18 quintillion)

### 6. The `collect()` Method
```rust
let parts: Vec<&str> = range.split('-').collect();
```
- Transforms an iterator into a collection
- Type annotation (`Vec<&str>`) tells `collect()` what to build
- Could also use: `let parts = range.split('-').collect::<Vec<&str>>()`

### 7. String vs &str - Understanding the Difference
```rust
let s = num.to_string();  // String (owned, heap-allocated)
let slice = &s[..];       // &str (borrowed, stack-allocated reference)
```

**The crucial distinction:**

| Type | Ownership | Memory | Growable | Use when... |
|------|-----------|--------|----------|-------------|
| `String` | Owned | Heap | Yes | You need to modify or own the data |
| `&str` | Borrowed | Stack (ref) | No | You just need to read the data |

**Common patterns:**
```rust
// Function that borrows (preferred for reading)
fn count_chars(s: &str) -> usize {  // Takes &str - works with String or &str
    s.len()
}

// Function that takes ownership (only if you need to)
fn into_uppercase(s: String) -> String {  // Takes String - consumes input
    s.to_uppercase()
}

// Using both:
let owned = String::from("hello");
count_chars(&owned);  // Borrow it
count_chars("world");  // String literal is already &str
```

**The Rust Trick - Prefer `&str` in function parameters:**
```rust
// ✓ Good - accepts both String and &str
fn process(s: &str) { }

// ✗ Less flexible - only accepts String
fn process(s: String) { }
```

**Why?** You can always pass a `&String` as `&str` (deref coercion), but not vice versa!

## Code Flow

1. Read input file (one line with comma-separated ranges)
2. Trim whitespace and split by commas
3. For each range:
   - Split by `-` to get start and end
   - Parse both as `u64`
   - Iterate through all numbers in range (inclusive)
   - Check if number is a repeated pattern
   - Add to total if invalid
4. Print the sum

## Example Walkthrough

**Range: `11-22`**
- Check 11: `"11"` → first="1", second="1" → Match! ✓
- Check 12: `"12"` → first="1", second="2" → No match
- Check 13-21: None match
- Check 22: `"22"` → first="2", second="2" → Match! ✓
- Invalid IDs: 11 + 22 = 33

**Range: `998-1012`**
- Check 998: `"998"` → odd length → Skip
- Check 999: `"999"` → odd length → Skip
- Check 1000: `"1000"` → first="10", second="00" → No match
- ...
- Check 1010: `"1010"` → first="10", second="10" → Match! ✓
- Invalid IDs: 1010

## Answer
**Sum of invalid IDs: 21,898,734,247**

## Rust Idioms & Best Practices Demonstrated

### 1. Zero-Cost Abstractions
String slicing gives you the convenience of high-level code with the performance of low-level code. The `&str` slice compiles down to just a pointer and length - no runtime overhead!

### 2. Prefer Borrowing Over Ownership
```rust
fn is_invalid_id(num: u64) -> bool  // Takes value (Copy type, so OK)
fn process_string(s: &str)          // Takes reference (borrowed, flexible)
```
For `Copy` types (like integers), passing by value is fine. For larger types, prefer borrowing.

### 3. Iterator Chains Over Loops
Rust encourages **functional programming** patterns:
```rust
// Imperative style (verbose)
let mut sum = 0;
for item in items {
    if item > 10 {
        sum += item * 2;
    }
}

// Functional style (idiomatic Rust)
let sum: i32 = items.iter()
    .filter(|&&x| x > 10)
    .map(|&x| x * 2)
    .sum();
```

### 4. Pattern Matching Philosophy
In future problems, you'll see `match` everywhere. Rust's philosophy: **make illegal states unrepresentable** and handle all cases explicitly.

### 5. The `?` Operator (Preview)
For error handling, Rust has the `?` operator (we'll see this more):
```rust
fn read_file() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("file.txt")?;  // Propagates error
    Ok(content)
}
```

## Common Rust Patterns You'll See

### Early Returns for Guard Clauses
```rust
if parts.len() != 2 {
    continue;  // Skip malformed data
}
```

### Type Annotations for Disambiguation
```rust
let parts: Vec<&str> = ...  // Tell compiler what type you want
```

### Method Chaining
```rust
input.trim().split(',')  // Fluent, readable
```

## Performance Tips

1. **Avoid unnecessary `collect()`** - Use iterators directly when possible
2. **Choose the right integer type** - Don't always default to `i32`
3. **Borrow instead of clone** - `&str` is cheaper than `String`
4. **Use inclusive ranges** (`..=`) when you need both endpoints

## Learning Insights

1. **String slicing** is a zero-cost abstraction for pattern analysis
2. **Inclusive ranges** (`..=`) include both endpoints - crucial for ranges
3. **Type sizes matter** - `u64` when dealing with large numbers/sums
4. **Iterators are lazy** - `split()` doesn't allocate until consumed
5. **Implicit returns** make code cleaner and more expression-oriented
6. **Prefer `&str`** in function parameters for maximum flexibility
