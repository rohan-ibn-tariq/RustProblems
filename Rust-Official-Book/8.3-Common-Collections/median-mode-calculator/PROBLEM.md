# Median and Mode Calculator

A Rust library that calculates statistical measures (median and mode) from a list of integers.

## Problem Definition

Given a vector of integers, implement a function that returns both:
- **Median**: The middle value when the list is sorted (or average of two middle values for even-length lists)
- **Mode**: The value that appears most frequently in the list

## Requirements

### Function Signature
```rust
fn calculate_stats(numbers: Vec<i32>) -> (f64, i32)
```

- **Input**: `Vec<i32>` - A vector of integers
- **Output**: `(f64, i32)` - A tuple containing (median, mode)

### Detailed Specifications

#### Median Calculation
- For odd-length vectors: return the middle element after sorting
- For even-length vectors: return the average of the two middle elements
- Return type: `f64` to handle averages

#### Mode Calculation  
- Find the value that occurs most frequently
- Use a `HashMap<i32, u32>` to count frequencies
- Return type: `i32` (the actual value, not the frequency)

### Edge Cases to Handle
- Empty vector: return appropriate default or error
- Single element: median and mode are the same value
- Multiple modes: return any one of them (or the first encountered)
- All elements appear once: return any element as mode

### Example Usage
```rust
let numbers = vec![1, 2, 2, 3, 4, 4, 4, 5];
let (median, mode) = calculate_stats(numbers);
println!("Median: {}, Mode: {}", median, mode);
// Expected: Median: 3.0, Mode: 4
```

## Test Cases to Implement
- `[1, 2, 3, 4, 5]` → median: 3.0, mode: any value
- `[1, 2, 2, 3, 4]` → median: 2.0, mode: 2
- `[1, 1, 2, 2, 3, 3]` → median: 2.0, mode: any of 1, 2, or 3
- `[5]` → median: 5.0, mode: 5