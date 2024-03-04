use std::collections::HashMap;

fn main() {
    let input = "1,2,3,4,5,6,2,7,8,9,9,10,11";
    let frequency = count_frequency(input);
    println!("{:?}", frequency);
}

fn count_frequency(input: &str) -> HashMap<String, u32> {
    let mut frequency: HashMap<String, u32> = HashMap::new();
    for c in input.split(",") {
        let counter = frequency.entry(c.to_string()).or_insert(0);
        *counter += 1;
    }
    frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_frequency_empty_input() {
        let input = "";
        let frequency = count_frequency(input);
        let mut expected: HashMap<String, u32> = HashMap::new();
        expected.insert("".to_string(), 1);
        assert_eq!(frequency, expected);
    }

    #[test]
    fn test_count_frequency_single_element() {
        let input = "1";
        let frequency = count_frequency(input);
        let mut expected: HashMap<String, u32> = HashMap::new();
        expected.insert("1".to_string(), 1);
        assert_eq!(frequency, expected);
    }

    #[test]
    fn test_count_frequency_multiple_elements() {
        let input = "1,2,3,4,5,6,2,7,8,9,9,10,11";
        let frequency = count_frequency(input);
        let mut expected: HashMap<String, u32> = HashMap::new();
        expected.insert("1".to_string(), 1);
        expected.insert("2".to_string(), 2);
        expected.insert("3".to_string(), 1);
        expected.insert("4".to_string(), 1);
        expected.insert("5".to_string(), 1);
        expected.insert("6".to_string(), 1);
        expected.insert("7".to_string(), 1);
        expected.insert("8".to_string(), 1);
        expected.insert("9".to_string(), 2);
        expected.insert("10".to_string(), 1);
        expected.insert("11".to_string(), 1);
        // println!("{:?}", expected);
        // println!("{:?}", frequency);
        assert_eq!(frequency, expected);
    }
}
