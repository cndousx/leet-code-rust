///
/// [442. 数组中重复的数据](https://leetcode.cn/problems/find-all-duplicates-in-an-array/?envType=problem-list-v2&envId=sorting)
///
struct Solution;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut result = vec![];
        let n = nums.len();
        for i in 0..n {
            // nums值的取值是[1,n]
            let num = nums[i].abs();
            let index = (num - 1) as usize;
            if nums[index] > 0 {
                // 取反，标记
                nums[index] *= -1;
            } else {
                result.push(num);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]);
        assert_eq!(ans, vec![2, 3])
    }

    #[test]
    fn t2() {
        let ans = Solution::find_duplicates(vec![1, 1, 2]);
        assert_eq!(ans, vec![1])
    }
}
