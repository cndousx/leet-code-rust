///
/// [918. 环形子数组的最大和](https://leetcode.cn/problems/maximum-sum-circular-subarray/?envType=problem-list-v2&envId=monotonic-queue)
///
struct Solution;
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();

        // Kadane 求最大子数组和
        let mut max_ending = nums[0];
        let mut max_so_far = nums[0];
        for &x in nums[1..].iter() {
            max_ending = x.max(max_ending + x);
            max_so_far = max_so_far.max(max_ending);
        }

        // Kadane 求最小子数组和
        let mut min_ending = nums[0];
        let mut min_so_far = nums[0];
        for &x in nums[1..].iter() {
            min_ending = x.min(min_ending + x);
            min_so_far = min_so_far.min(min_ending);
        }

        if total == min_so_far {
            // 如果所有数都是负数，则环形最大就是普通最大（即单个最大负数）
            max_so_far
        } else {
            max_so_far.max(total - min_so_far)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3)
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -2, 5]), 10)
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![3, -2, 2, -3]), 3)
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![3, -1, 2, -1]), 4)
    }
}
