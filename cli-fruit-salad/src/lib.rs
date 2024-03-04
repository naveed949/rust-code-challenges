use rand::Rng;


pub fn create_fruit_salad(mut number: u32) -> Option<String> {
    let fruits = vec![
        "apple", "banana", "orange", "grape", "kiwi", "mango", "pear", "pineapple", "strawberry", "watermelon"
    ];

    if number > 0 && number <= fruits.len() as u32 {
        let mut salad: Vec<String> = Vec::new();
        let mut rng = rand::thread_rng();
        while number > 0 {
            let random_index = rng.gen_range(0..fruits.len());
            let fruit = fruits[random_index].to_string();
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
}
