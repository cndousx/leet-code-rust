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
        if n == 2 {
            return nums[0].max(nums[1]);
        }

        // 线性打家劫舍函数（空间优化版）
        let rob_linear = |start: usize, end: usize| -> i32 {
            let mut prev2 = 0;
            let mut prev1 = 0;
            for i in start..=end {
                let curr = prev1.max(prev2 + nums[i]);
                prev2 = prev1;
                prev1 = curr;
            }
            prev1
        };

        // 两种情况取最大值
        // 情况1：不偷第一家 → 可以偷最后一家
        // 情况2：不偷最后一家 → 可以偷第一家
        rob_linear(1, n - 1).max(rob_linear(0, n - 2))
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
