///
/// [137. 只出现一次的数字Ⅱ](https://leetcode.cn/problems/single-number-ii/)
///
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // 记录当前每个二进制位上出现 奇数次（1 次） 的状态
        let mut ones = 0;
        // 记录当前每个二进制位上出现 偶数次（2 次） 的状态
        let mut twos = 0;
        for num in nums {
            ones = ones ^ num & !twos;
            twos = twos ^ num & !ones;
        }
        ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
