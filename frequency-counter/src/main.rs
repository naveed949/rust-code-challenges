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
        assert_eq!(frequency, expected);
    }
    #[test]
    fn test_count_frequency_words() {
        let input = "apple,banana,apple,orange,banana,apple";
        let frequency = count_frequency(input);
        let mut expected: HashMap<String, u32> = HashMap::new();
        expected.insert("apple".to_string(), 3);
        expected.insert("banana".to_string(), 2);
        expected.insert("orange".to_string(), 1);
        assert_eq!(frequency, expected);
    }

    #[test]
    fn test_count_frequency_mixed() {
        let input = "1,2,3,apple,banana,apple,orange,banana,apple,4,5,6,2,7,8,9,9,10,11";
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
        expected.insert("apple".to_string(), 3);
        expected.insert("banana".to_string(), 2);
        expected.insert("orange".to_string(), 1);
        assert_eq!(frequency, expected);
    }

    #[test]
    fn test_count_frequency_chars() {
        let input = "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,c,d,e,f,g,h,i,j,s,t,u,v,w";
        let frequency = count_frequency(input);
        let mut expected: HashMap<String, u32> = HashMap::new();
        expected.insert("a".to_string(), 1);
        expected.insert("b".to_string(), 1);
        expected.insert("c".to_string(), 2);
        expected.insert("d".to_string(), 2);
        expected.insert("e".to_string(), 2);
        expected.insert("f".to_string(), 2);
        expected.insert("g".to_string(), 2);
        expected.insert("h".to_string(), 2);
        expected.insert("i".to_string(), 2);
        expected.insert("j".to_string(), 2);
        expected.insert("k".to_string(), 1);
        expected.insert("l".to_string(), 1);
        expected.insert("m".to_string(), 1);
        expected.insert("n".to_string(), 1);
        expected.insert("o".to_string(), 1);
        expected.insert("p".to_string(), 1);
        expected.insert("q".to_string(), 1);
        expected.insert("r".to_string(), 1);
        expected.insert("s".to_string(), 2);
        expected.insert("t".to_string(), 2);
        expected.insert("u".to_string(), 2);
        expected.insert("v".to_string(), 2);
        expected.insert("w".to_string(), 2);
        expected.insert("x".to_string(), 1);
        expected.insert("y".to_string(), 1);
        expected.insert("z".to_string(), 1);
        assert_eq!(frequency, expected);
    }
}
