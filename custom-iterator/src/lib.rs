
/// An iterator that generates Fibonacci numbers.
///
/// This iterator generates Fibonacci numbers starting from 0 and 1.
/// Each subsequent number is the sum of the two previous numbers.
pub struct Fibonacci {
    current: u32,
    next: u32,
}

impl Fibonacci {
     pub fn new() -> Fibonacci {
         Fibonacci {
             current: 0,
             next: 1,
         }
     }
}
impl Iterator for Fibonacci {
    type Item = u32;

    /// Generates the next Fibonacci number in the sequence.
    ///
    /// # Returns
    ///
    /// - `Some(number)`: The next Fibonacci number in the sequence.
    /// - `None`: If the sequence has reached its end. (The sequence is infinite, so this will never happen)
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;

        self.current = std::mem::replace(&mut self.next, new_next);

        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new();

        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));

    }
}