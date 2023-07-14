

fn unique(a: Vec<i32>) -> Vec<i32> {
    let mut b = a.clone();
    b.sort_by(|a, b| a.partial_cmp(b).unwrap());
    b.dedup();
    b
}

// advanced 1: use generic types
fn unique_by_generics<T> (a: Vec<T>) -> Vec<T> where T: std::cmp::Ord + std::clone::Clone {
    let mut b = a.clone();
    b.sort_by(|a, b| a.partial_cmp(b).unwrap());
    b.dedup();
    b
}

// advanced 2: keep items in order
fn uniqueness_with_order<T>(a: Vec<T>) -> Vec<T> where T: std::cmp::Ord + std::clone::Clone {
    let mut b = a.clone();

    b.iter().enumerate().for_each(|(i, _)| {
        let mut j = i + 1;
        while j < b.len() {
            if b[i] == b[j] {
                b.remove(j);
            } else {
                j += 1;
            }
        }
    });

    b
}

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}


#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}