use std::collections::HashMap;
///
/// [638. 大礼包](https://leetcode.cn/problems/shopping-offers/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let n = price.len();

        // 过滤无效礼包
        let valid_specials: Vec<Vec<i32>> = special
            .into_iter()
            .filter(|sp| {
                if sp[..n].iter().all(|&x| x <= 0) {
                    // 礼包内至少有一种物品数量 > 0
                    // 礼包中物品数量全为0,过滤掉
                    return false;
                }

                let single_cost = sp[..n]
                    .iter()
                    .enumerate()
                    .map(|(i, &count)| count * price[i])
                    .sum::<i32>();
                // 礼包价格 > 单独购买的总价，则该礼包是不在购入范围
                single_cost > sp[n]
            })
            .collect();

        let mut memo: HashMap<Vec<i32>, i32> = HashMap::new();

        fn dfs(
            cur_needs: &[i32],
            price: &[i32],
            valid_specials: &[Vec<i32>],
            memo: &mut HashMap<Vec<i32>, i32>,
        ) -> i32 {
            // 查缓存
            if let Some(&cached) = memo.get(cur_needs) {
                return cached;
            }

            let n = price.len();

            // 基准价：直接按单价购买
            let mut cost: i32 = cur_needs
                .iter()
                .enumerate()
                .map(|(i, &count)| count * price[i])
                .sum();

            // 尝试每个有效礼包
            for sp in valid_specials {
                if !(0..n).all(|i| cur_needs[i] >= sp[i]) {
                    // 检查是否可以购买该礼包
                    // 即当前需要购买的数量必须大于等于礼包中的物品数量(不能买超)
                    continue;
                }

                // 生成新的需求状态
                let new_needs = (0..n).map(|i| cur_needs[i] - sp[i]).collect::<Vec<_>>();
                let special_cost = sp[n];
                let sub_cost = special_cost + dfs(&new_needs, price, valid_specials, memo);
                //  更新最小值
                cost = cost.min(sub_cost);
            }

            // 存入缓存
            memo.insert(cur_needs.to_vec(), cost);
            cost
        }

        dfs(&needs, &price, &valid_specials, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // 有 A 和 B 两种物品，价格分别为 ¥2 和 ¥5 。
        // 大礼包 1 ，你可以以 ¥5 的价格购买 3A 和 0B 。
        // 大礼包 2 ，你可以以 ¥10 的价格购买 1A 和 2B 。
        // 需要购买 3 个 A 和 2 个 B ， 所以付 ¥10 购买 1A 和 2B（大礼包 2），以及 ¥4 购买 2A 。
        let ans =
            Solution::shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2]);
        assert_eq!(ans, 14)
    }

    #[test]
    fn t2() {
        let ans = Solution::shopping_offers(
            vec![2, 3, 4],
            vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
            vec![1, 2, 1],
        );
        assert_eq!(ans, 11)
    }
}
