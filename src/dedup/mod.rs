/// Returns a new `Vec<i32>` with duplicates removed from the input `Vec<i32>`.
///
/// The `dedup` function takes a `Vec<i32>` as input and returns a new `Vec<i32>` with duplicates removed. The input `Vec<i32>` is not modified.
///
/// # Examples
///
/// ```
/// use rust_code_challenges::dedup::dedup;
///
/// let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 5];
/// let expected_output = vec![1, 2, 3, 4, 5];
/// let output = dedup(input);
/// assert_eq!(output, expected_output);
/// ```
///
/// # Performance
///
/// The `dedup` function has a time complexity of O(n log n) due to the use of the `sort_by` method. The space complexity is O(n) due to the creation of a new `Vec<i32>` to store the output. If the input `Vec<i32>` is already sorted, the time complexity can be reduced to O(n) by using the `dedup_by` method instead of `sort_by` and `dedup`.
///
/// # Panics
///
/// The `dedup` function does not panic under normal circumstances. However, if the input `Vec<i32>` contains NaN values, the `partial_cmp` method used by `sort_by` will panic. It is the responsibility of the caller to ensure that the input `Vec<i32>` does not contain NaN values.
///
/// # Safety
///
/// The `dedup` function is safe to use as long as the input `Vec<i32>` does not contain NaN values. It does not use any unsafe Rust features or external dependencies.
pub fn dedup(a: Vec<i32>) -> Vec<i32> {
    let mut b = a.clone();
    b.sort_by(|a, b| a.partial_cmp(b).unwrap());
    b.dedup();
    b
}

/// Returns a new `Vec<i32>` with duplicates removed from the input `Vec<i32>` while preserving the order of the remaining elements.
///
/// The `dedup_preserve_order` function takes a `Vec<i32>` as input and returns a new `Vec<i32>` with duplicates removed while preserving the order of the remaining elements. The input `Vec<i32>` is not modified.
///
/// # Examples
///
/// ```
/// use rust_code_challenges::dedup::dedup_preserve_order;
///
/// let input = vec![1, 2, 2, 5, 3, 3, 3, 4, 4, 6, 7, 7];
/// let expected_output = vec![1, 2, 5, 3, 4, 6, 7];
/// let output = dedup_preserve_order(input);
/// assert_eq!(output, expected_output);
/// ```
///
/// # Performance
///
/// The `dedup_preserve_order` function has a time complexity of O(n) due to the use of a `HashMap` to keep track of unique elements. The space complexity is also O(n) due to the creation of a new `Vec<i32>` to store the output and a `HashMap` to keep track of unique elements.
///
/// # Panics
///
/// The `dedup_preserve_order` function does not panic under normal circumstances.
///
/// # Safety
///
/// The `dedup_preserve_order` function is safe to use. It does not use any unsafe Rust features or external dependencies.
pub fn dedup_preserve_order<T>(a: Vec<T>) -> Vec<T>
where
    T: std::cmp::Ord + std::clone::Clone + std::hash::Hash,
{
    let mut b: Vec<T> = Vec::new();

    // declare hash map to store unique items
    let mut unique_items = std::collections::HashMap::new();

    // iterate over items
    for item in a.iter() {
        // check if item is in hash map
        if !unique_items.contains_key(item) {
            // if not, add it
            unique_items.insert(item.clone(), true);
            // and add it to the output vector
            b.push(item.clone());
        }
    }
    b
}

#[cfg(test)]
mod dedup_tests {
    use super::*;
    #[test]
    fn empty_list() {
        let input = vec![];
        let expected_output = vec![];
        let actual_output = dedup(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list() {
        let input = vec![1, 4, 5];
        let expected_output = vec![1, 4, 5];
        let actual_output = dedup(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list() {
        let input = vec![1, 5, 2];
        let expected_output = vec![1, 2, 5];
        let actual_output = dedup(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list_with_duplicates() {
        let input = vec![1, 5, 2, 2, 1];
        let expected_output = vec![1, 2, 5];
        let actual_output = dedup(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list_with_duplicates() {
        let mut input = vec![1, 5, 2, 2, 1];
        input.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let expected_output = vec![1, 2, 5];
        let actual_output = dedup(input);
        assert_eq!(actual_output, expected_output);
    }
}

#[cfg(test)]
mod dedup_preserve_order_tests {
    use super::*;
    #[test]
    fn empty_list() {
        let input: Vec<i32> = vec![];
        let expected_output = vec![];
        let actual_output = dedup_preserve_order(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list() {
        let input = vec![1, 4, 5];
        let expected_output = vec![1, 4, 5];
        let actual_output = dedup_preserve_order(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list() {
        let input = vec![1, 5, 2];
        let expected_output = vec![1, 5, 2];
        let actual_output = dedup_preserve_order(input);
        assert_eq!(actual_output, expected_output);
    }
    #[test]
    fn sorted_list_with_duplicates() {
        let mut input = vec![1, 1, 2, 3, 3, 4, 5, 5];
        input.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let expected_output = vec![1, 2, 3, 4, 5];
        let actual_output = dedup_preserve_order(input);
        assert_eq!(actual_output, expected_output);
    }
    #[test]
    fn unsorted_list_with_duplicates() {
        let input = vec![1, 5, 2, 2, 1];
        let expected_output = vec![1, 5, 2];
        let actual_output = dedup_preserve_order(input);
        assert_eq!(actual_output, expected_output);
    }
}
