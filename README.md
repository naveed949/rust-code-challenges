# Rust Code Challenges

This repository contains solutions to various Rust code challenges. Each challenge is implemented as a separate Rust module and includes a set of unit tests to verify the correctness of the implementation.

## Challenges

- [Median](#median)
- [Dedup](#dedup)

### Median

The `median` module contains a function to calculate the median of a vector of floating-point numbers.

```rust
use rust_code_challenges::median::median;

let input = vec![1.0, 2.0, 5.0];
let expected_output = Some(2.0);
let actual_output = median(input);
assert_eq!(actual_output, expected_output);
```

### Dedup

The `dedup` module contains two function to remove duplicates from a vector of integers.

1. `dedup` - Removes duplicates from a vector of integers.
2. `dedup_preserve_order` - Removes duplicates from a vector of integers while preserving the order of the remaining elements.

```rust
use rust_code_challenges::dedup::dedup;

let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 5];
let expected_output = vec![1, 2, 3, 4, 5];
let actual_output = dedup(input);
assert_eq!(actual_output, expected_output);
```

```rust
use rust_code_challenges::dedup::dedup_preserve_order;

let input = vec![1, 2, 2, 4, 3, 3, 3, 4, 4, 7, 7, 5];
let expected_output = vec![1, 2, 4, 3, 7, 5];
let actual_output = dedup_preserve_order(&input);
assert_eq!(actual_output, expected_output);
```

## Contributing

Contributions to this repository are welcome! If you have a code challenge you'd like to see implemented in Rust, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
