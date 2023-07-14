/// Calculates the median of a vector of f32 values.
///
/// # Arguments
///
/// * `a` - A vector of f32 values.
///
/// # Returns
///
/// * `None` if the input vector is empty.
/// * The median value of the input vector as a `Some(f32)` otherwise.
///
/// # Examples
///
/// ```
/// use rust_code_challenges::median::median;
///
/// let input = vec![1.0, 2.0, 5.0];
/// let expected_output = Some(2.0);
/// let actual_output = median(input);
/// assert_eq!(actual_output, expected_output);
/// ```
pub fn median(a: Vec<f32>) -> Option<f32> {
    if a.len() == 0 {
        return None;
    }

    let mut b = a.clone();
    b.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = b.len() / 2;
    if b.len() % 2 == 0 {
        Some((b[mid - 1] + b[mid]) / 2.0)
    } else {
        Some(b[mid])
    }
}
#[cfg(test)]
mod median_tests {
    use super::*;
    #[test]
    fn empty_list() {
        let input = vec![];
        let expected_output = None;
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list() {
        let input = vec![1.0, 4.0, 5.0];
        let expected_output = Some(4.0);
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn even_length() {
        let input = vec![1.0, 3.0, 5.0, 6.0];
        let expected_output = Some(4.0);
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list() {
        let input = vec![1.0, 5.0, 2.0];
        let expected_output = Some(2.0);
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }
}
