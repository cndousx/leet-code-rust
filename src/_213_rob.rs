///
/// [213. 打家劫舍 Ⅱ](https://leetcode.cn/problems/house-robber-ii/)
///
struct Solution;
impl Solution {
    ///
    ///
    /// 分别计算两种情况的最大值：
    /// - 不偷第一家：从 [1, n-1] 范围内偷盗
    /// - 不偷最后一家：从 [0, n-2] 范围内偷盗
    /// 返回这两种情况的最大值
    ///
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }

        let mut max = i32::MIN;
        for (start, end) in vec![(1, n - 1), (0, n - 2)] {
            let mut dp = vec![0; n];
            dp[start] = nums[start];
            for i in start + 1..=end {
                dp[i] = dp[(n + i - 1) % n].max(dp[(n + i - 2) % n] + nums[i]);
            }
            max = max.max(dp[end]);
        }

        return max;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::rob(vec![1, 2, 3, 1]);
        assert_eq!(ans, 4);
    }

    #[test]
    fn t2() {
        let ans = Solution::rob(vec![2, 3, 2]);
        assert_eq!(ans, 3);
    }

    #[test]
    fn t3() {
        let ans = Solution::rob(vec![2, 3, 2]);
        assert_eq!(ans, 3);
    }

    #[test]
    fn t4() {
        let ans = Solution::rob(vec![1, 2, 3]);
        assert_eq!(ans, 3);
    }

    #[test]
    fn t5() {
        let ans = Solution::rob(vec![200, 3, 140, 20, 10]);
        assert_eq!(ans, 340);
    }

    #[test]
    fn t6() {
        let ans = Solution::rob(vec![0]);
        assert_eq!(ans, 0);
    }
}
