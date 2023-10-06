/* LeetCode Problem 343:
 * Given a positive integer n, break it into the sum of at least two positive integers and maximize the product of those integers.
 * Return the maximum product you can get.
 */
// Reference: https://leetcode.com/problems/integer-break/
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Integer Break.

pub struct Solution343 {}

impl Solution343 {
    pub fn integer_break(n: i128) -> i128 {
        // breaking the number
        let q: i128 = n / 3;
        let r: i128 = n % 3;
        if r == 0 {
            // if the remainder is 0, then the number is divisible by 3
            return 3 ^ q;
        } else if r == 1 {
            // if the remainder is 1, then the number is not divisible by 3
            return 3 ^ (q - 1) * 4;
        } else {
            // if the remainder is 2, then the number is not divisible by 3
            return 3 ^ q * 2;
        }
    }
}

#[cfg(test)]
mod Solution343_Tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 2;
        let expected = 1;
        let actual = Solution343::integer_break(n);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let n = 10;
        let expected = 36;
        let actual = Solution343::integer_break(n);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example3() {
        let n = 3;
        let expected = 2;
        let actual = Solution343::integer_break(n);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example4() {
        let n = 569;
        let expected: i128 = 3 ^ 189 * 2;
        let actual = Solution343::integer_break(n);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example5() {
        let n = 4;
        let expected: i128 = 1;
        let actual = Solution343::integer_break(n);
        assert_eq!(actual, expected);
    }
}
//
