// leetcode 2251
/*
* You are given a 0-indexed 2D integer array flowers, where flowers[i] = [starti, endi] means the ith flower will be in full bloom from starti to endi (inclusive).
* You are also given a 0-indexed integer array people of size n, where people[i] is the time that the ith person will arrive to see the flowers.
** Return an integer array answer of size n, where answer[i] is the number of flowers that are in full bloom when the ith person arrives.
*/
struct FlowersBloom;

impl FlowersBloom {
    pub fn full_bloom_flowers_v2(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut flowers_bloom = Vec::new();
        // sort flowers by start time in ascending order
        let mut flowers = flowers;
        flowers.sort_by(|a, b| a[0].cmp(&b[0]));
        // start indexes
        let mut start_indexes: Vec<usize> = Vec::new();
        // merge intervals
        let mut merged_intervals = FlowersBloom::merge(&flowers, &mut start_indexes);
        println!("merged_intervals: {:?}", merged_intervals);

        for p in people {
            let mut merge_index = 0;
            // find the merge interval that contains p
            for i in 0..merged_intervals.len() {
                if merged_intervals[i][0] <= p && merged_intervals[i][1] >= p {
                    merge_index = i;
                    break;
                }
            }
            let mut count = 0;
            let start_index = start_indexes[merge_index];
            for i in start_index..flowers.len() {
                if merged_intervals[i][0] <= p && merged_intervals[i][1] >= p {
                    count += 1;
                } else if merged_intervals[i][0] > p {
                    break;
                }
            }
            flowers_bloom.push(count);
        }
        flowers_bloom
    }
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut flowers_bloom = Vec::new();
        // prepare data for binary search
        let mut flowers = flowers;
        flowers.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut count = 0;
        let mut start: usize = 0;
        for p in people {
            start = FlowersBloom::start_index(&flowers, p);
            println!("start: {}", start);
            count = 0;
            for f in start..flowers.len() {
                if flowers[f][0] > p {
                    break;
                } else if flowers[f][1] >= p {
                    count += 1;
                }
            }
            flowers_bloom.push(count);
        }
        flowers_bloom
    }

    fn start_index(flowers: &Vec<Vec<i32>>, p: i32) -> usize {
        let mut end = flowers.len();
        let mut mid = end / 2;
        while mid > 0 {
            if flowers[mid][0] > p && flowers[mid][1] != p {
                end = mid;
                mid = end / 2;
            } else if flowers[mid][0] < p && flowers[mid][1] != p {
                // [..., mid,[...]] needs to be fixed
                println!("end: {}", end);
                println!("mid: {}", mid);
                mid = (end + mid) / 2; // can overflow
                println!("calc::mid: {}", mid);
            } else {
                return mid;
            }
        }
        mid
    }
    fn merge(flowers: &Vec<Vec<i32>>, start_indexes: &mut Vec<usize>) -> Vec<Vec<i32>> {
        let mut merged_intervals = Vec::new();
        let mut i = 0;
        while i < flowers.len() {
            let mut interval = vec![flowers[i][0], flowers[i][1]];
            start_indexes.push(i); // log the start index of current merge interval
            i += 1;
            for j in i..flowers.len() {
                if flowers[j][0] <= interval[1] {
                    if interval[1] < flowers[j][1] {
                        interval[1] = flowers[j][1];
                    }
                    interval[1] = flowers[j][1];
                } else {
                    i = j - 1;
                    break;
                }
            }
            merged_intervals.push(interval);
        }

        merged_intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_bloom_flowers() {
        let flowers = vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]];
        let people = vec![2, 3, 7, 11];
        let flowers_bloom = FlowersBloom::full_bloom_flowers_v2(flowers, people);
        assert_eq!(flowers_bloom, vec![1, 2, 2, 2]);
    }

    #[test]
    fn test_full_bloom_flowers_2() {
        let flowers = vec![
            vec![5, 8],
            vec![1, 7],
            vec![2, 8],
            vec![3, 10],
            vec![4, 9],
            vec![6, 9],
            vec![3, 8],
            vec![4, 7],
            vec![5, 10],
            vec![1, 9],
        ];
        let people = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let flowers_bloom = FlowersBloom::full_bloom_flowers(flowers, people);
        assert_eq!(flowers_bloom, vec![2, 3, 5, 7, 9, 10, 10, 8, 5, 2]);
    }
}
