///
/// [123. 买卖股票的最佳时机Ⅲ](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iii/description/)
///
struct Solution;
impl Solution {
    /// 动态规划+状态机
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 第1次买入后的最大利润
        let mut buy1 = i32::MIN;
        // 第1次卖出后，最大利润
        let mut sell1 = i32::MIN;
        // 第2次买入后，最大利润
        let mut buy2 = i32::MIN;
        // 第2次卖出后，最大利润
        let mut sell2 = i32::MIN;
        for price in prices {
            // 第1次买入，利润是0-price
            buy1 = buy1.max(-price);
            sell1 = sell1.max(buy1 + price);
            buy2 = buy2.max(sell1 - price);
            sell2 = sell2.max(buy2 + price);
        }
        sell2
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4)
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0)
    }
}
