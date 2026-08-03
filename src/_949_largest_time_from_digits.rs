///
/// [949. 给定数字能组成的最大时间](https://leetcode.cn/problems/largest-time-for-given-digits/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut arr = arr;
        let mut largest: Vec<i32> = vec![];
        Solution::dfs(&mut largest, &mut arr, 0, 3);
        if largest.is_empty() {
            "".to_string()
        } else {
            format!("{}{}:{}{}", largest[0], largest[1], largest[2], largest[3])
        }
    }

    fn dfs(largest: &mut Vec<i32>, arr: &mut Vec<i32>, start: usize, end: usize) {
        if start == end && Solution::is_valid_time(arr) {
            if largest.is_empty() {
                for e in arr {
                    largest.push(*e);
                }
            } else {
                let sum1 = (largest[0] * 10 + largest[1]) * 60 + largest[2] * 10 + largest[3];
                let sum2 = (arr[0] * 10 + arr[1]) * 60 + arr[2] * 10 + arr[3];
                if sum2 > sum1 {
                    largest.clear();
                    for e in arr {
                        largest.push(*e);
                    }
                }
            }
            return;
        }
        for i in start..=end {
            arr.swap(start, i);
            Solution::dfs(largest, arr, start + 1, end);
            arr.swap(start, i);
        }
    }

    fn is_valid_time(arr: &Vec<i32>) -> bool {
        if arr.is_empty() {
            return false;
        }
        // HH:mm
        // HH 取值是00到23
        // mm 取值是00到59
        if arr.len() != 4 {
            return false;
        }

        let hh = arr[0] * 10 + arr[1];
        if hh > 23 || hh < 0 {
            return false;
        }
        let mm = arr[2] * 10 + arr[3];
        if mm > 59 || mm < 0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![1, 2, 3, 4]),
            "23:41".to_string()
        )
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![5, 5, 5, 5]),
            "".to_string()
        )
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![0, 0, 0, 0]),
            "00:00".to_string()
        )
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![0, 0, 1, 0]),
            "10:00".to_string()
        )
    }
}
