///
/// [121. 买卖股票的最佳时机](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/)
///
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;
        for price in prices {
            if price < min_price {
                min_price = price;
            }
            max_profit = max_profit.max(price - min_price)
        }
        max_profit
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
    }

    #[test]
    fn t2() {
        let result = Solution::max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(result, 0);
    }
}
