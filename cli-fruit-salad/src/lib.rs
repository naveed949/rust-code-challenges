use std::collections::{LinkedList, VecDeque};

use rand::Rng;
use rand::seq::SliceRandom;


pub fn create_fruit_salad(mut number: u32) -> Option<String> {
    let fruits = vec![
        "apple", "banana", "orange", "grape", "kiwi", "mango", "pear", "pineapple", "strawberry", "watermelon"
    ];

    if number > 0 && number <= fruits.len() as u32 {
        let mut salad: Vec<String> = Vec::new();
        let mut rng = rand::thread_rng();
        while number > 0 {
            // let random_index = rng.gen_range(0..fruits.len());
            let fruit = fruits.choose(&mut rng).unwrap().to_string();
            if !salad.contains(&fruit) {
                salad.push(fruit.to_string());
                number -= 1;
            }
        }
        Some(salad.join(", "))
    } else {
        None
    }
}

pub fn create_fruit_salad_with_linkedlist(mut number: u32) -> Option<String> {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("apple");
    fruit.push_back("banana");
    fruit.push_back("orange");
    fruit.push_back("grape");
    fruit.push_back("kiwi");
    fruit.push_back("mango");
    fruit.push_back("pear");
    fruit.push_back("pineapple");
    fruit.push_back("strawberry");
    fruit.push_back("watermelon");

    if number > 0 && number <= fruit.len() as u32 {
        let mut salad: Vec<String> = Vec::new();
        let mut rng = rand::thread_rng();
        while number > 0 {
            let random_index = rng.gen_range(0..fruit.len());
            let fruit = fruit.iter().nth(random_index).unwrap();
            if !salad.contains(&fruit.to_string()) {
                salad.push(fruit.to_string());
                number -= 1;
            }
        }
        Some(salad.join(", "))
    } else {
        None
    }
}

pub fn create_fruit_salad_with_vecdeque(mut number: u32) -> Option<String> {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("apple");
    fruit.push_back("banana");
    fruit.push_back("orange");
    fruit.push_back("grape");
    fruit.push_back("kiwi");
    fruit.push_back("mango");
    fruit.push_back("pear");
    fruit.push_back("pineapple");
    fruit.push_back("strawberry");
    fruit.push_back("watermelon");

    if number > 0 && number <= fruit.len() as u32 {
        let mut salad: Vec<String> = Vec::new();
        let mut rng = rand::thread_rng();
        while number > 0 {
            let random_index = rng.gen_range(0..fruit.len());
            let fruit = fruit.get(random_index).unwrap();
            if !salad.contains(&fruit.to_string()) {
                salad.push(fruit.to_string());
                number -= 1;
            }
        }
        Some(salad.join(", "))
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fruit_salad_with_valid_number() {
        let number = 5;
        let result = create_fruit_salad(number);
        assert!(result.is_some());
        let fruit = result.unwrap();
        let fruit_count = fruit.split(", ").count();
        assert!(fruit_count == 5);
    }

    #[test]
    fn test_create_fruit_salad_with_zero_number() {
        let number = 0;
        let result = create_fruit_salad(number);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_fruit_salad_with_large_number() {
        let number = 100;
        let result = create_fruit_salad(number);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_fruit_salad_multiple_times() {
        let number = 3;
        for _ in 0..10 {
            let result = create_fruit_salad(number);
            assert!(result.is_some());
            let fruit = result.unwrap();
            let fruit = fruit.split(", ").collect::<Vec<&str>>();
            assert!(fruit.len() == 3);
        }
    }

    #[test]
    fn test_create_fruit_salad_with_linkedlist_with_valid_number() {
        let number = 5;
        let result = create_fruit_salad_with_linkedlist(number);
        assert!(result.is_some());
        let fruit = result.unwrap();
        let fruit_count = fruit.split(", ").count();
        assert!(fruit_count == 5);
    }

    #[test]
    fn test_create_fruit_salad_with_linkedlist_with_zero_number() {
        let number = 0;
        let result = create_fruit_salad_with_linkedlist(number);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_fruit_salad_with_linkedlist_with_large_number() {
        let number = 100;
        let result = create_fruit_salad_with_linkedlist(number);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_fruit_salad_with_linkedlist_multiple_times() {
        let number = 3;
        for _ in 0..10 {
            let result = create_fruit_salad_with_linkedlist(number);
            assert!(result.is_some());
            let fruit = result.unwrap();
            let fruit = fruit.split(", ").collect::<Vec<&str>>();
            assert!(fruit.len() == 3);
        }
    }

    #[test]
    fn test_create_fruit_salad_with_vecdeque_with_valid_number() {
        let number = 5;
        let result = create_fruit_salad_with_vecdeque(number);
        assert!(result.is_some());
        let fruit = result.unwrap();
        let fruit_count = fruit.split(", ").count();
        assert!(fruit_count == 5);
    }

    #[test]
    fn test_create_fruit_salad_with_vecdeque_with_zero_number() {
        let number = 0;
        let result = create_fruit_salad_with_vecdeque(number);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_fruit_salad_with_vecdeque_with_large_number() {
        let number = 100;
        let result = create_fruit_salad_with_vecdeque(number);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_fruit_salad_with_vecdeque_multiple_times() {
        let number = 3;
        for _ in 0..10 {
            let result = create_fruit_salad_with_vecdeque(number);
            assert!(result.is_some());
            let fruit = result.unwrap();
            let fruit = fruit.split(", ").collect::<Vec<&str>>();
            assert!(fruit.len() == 3);
        }
    }
    
}
