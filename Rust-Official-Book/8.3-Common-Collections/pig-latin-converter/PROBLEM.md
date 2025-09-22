# Pig Latin Converter

A Rust library that converts English strings to Pig Latin following traditional rules.

## Problem Definition

Convert English words to Pig Latin using these transformation rules:
- **Consonant-starting words**: Move the first consonant to the end and add "ay"
  - Example: "first" → "irst-fay"
- **Vowel-starting words**: Add "hay" to the end
  - Example: "apple" → "apple-hay"

## Requirements

### Function Signature
```rust
fn to_pig_latin(input: &str) -> String
```

- **Input**: `&str` - A string containing English words
- **Output**: `String` - The converted Pig Latin string

### Detailed Specifications

#### Vowel Identification
- Vowels: `a`, `e`, `i`, `o`, `u` (case-insensitive)
- Consider both uppercase and lowercase vowels
- Y is considered a consonant for this problem

#### Word Processing Rules
1. **Consonant words**: Move first consonant + add "ay"
   - "hello" → "ello-hay" (h is moved)
   - "string" → "tring-say" (s is moved)
   
2. **Vowel words**: Keep original + add "hay"
   - "apple" → "apple-hay"
   - "elephant" → "elephant-hay"

#### Multi-word Handling
- Process each word separately
- Preserve spaces between words, but remove trailing and leading white space in the full string and limit the in-between spaces to 1.
- "hello world" → "ello-hay orld-way"

### UTF-8 Encoding Considerations
- Handle proper character boundaries
- Use `chars()` iterator instead of byte indexing
- Preserve non-ASCII characters appropriately
- Consider that some characters might be multi-byte

### Edge Cases to Handle
- Empty strings
- Single character words
- Words with first char not an alphabetic character (keep as is)
- Mixed case preservation
- Leading/trailing whitespace (remove)
- Multiple consecutive spaces (reduce to 1)

### Example Usage
```rust
let english = "hello world";
let pig_latin = to_pig_latin(english);
println!("{}", pig_latin);
// Expected output: "ello-hay orld-way"

let vowel_start = "apple orange";
let result = to_pig_latin(vowel_start);
println!("{}", result);
// Expected output: "apple-hay orange-hay"
```

## Test Cases to Implement
- `"first"` → `"irst-fay"`
- `"apple"` → `"apple-hay"`
- `"hello world"` → `"ello-hay orld-way"`
- `"eat pie"` → `"eat-hay ie-pay"`
- `""` → `""`
- `"a"` → `"a-hay"`
- `"b"` → `"b-ay"`
