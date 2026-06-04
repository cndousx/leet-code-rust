///
/// [198. 打家劫舍](https://leetcode.cn/problems/house-robber/)
///
struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return nums[0].max(nums[1]);
        }
        let mut max = i32::MIN;
        for start in 0..=1 {
            let mut dp = vec![0; n + 1];
            dp[start] = nums[start];
            for i in start + 1..n {
                if i == 1 {
                    // 说明start==0，1需要跳过
                    continue;
                }
                dp[i] = if nums[i] > nums[i - 1] {
                    // 当前值，比前一个值大，丢弃前一个值，取当前值
                    dp[i - 1] - nums[i - 1] + nums[i]
                } else {
                    dp[i - 1]
                }
                .max(dp[i - 2] + nums[i]);
            }
            max = max.max(dp[n - 1]);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::rob(vec![2, 1]), 2);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }
}
