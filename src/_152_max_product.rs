///
/// [152. 乘积最大子数组](https://leetcode.cn/problems/maximum-product-subarray/)
///
struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp_max = vec![0; n];
        let mut dp_min = vec![0; n];

        dp_max[0] = nums[0];
        dp_min[0] = nums[0];
        let mut max = nums[0];

        for i in 1..n {
            let num = nums[i];

            // 计算以当前元素结尾的最大和最小乘积
            dp_max[i] = num.max(num * dp_max[i - 1]).max(num * dp_min[i - 1]);
            dp_min[i] = num.min(num * dp_max[i - 1]).min(num * dp_min[i - 1]);

            // 全局最大值
            max = max.max(dp_max[i]);
        }

        max
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
}
