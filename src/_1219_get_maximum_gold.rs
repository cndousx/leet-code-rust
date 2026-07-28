///
/// [1219. 黄金旷工](https://leetcode.cn/problems/path-with-maximum-gold/description/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    /// 提示
    /// - `1 <= grid.length, grid[i].length <= 15`
    /// - `0 <= grid[i][j] <= 100`
    /// - 最多 25 个单元格中有黄金。
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut max_gold = 0;
        for i in 0..m {
            for j in 0..n {
                let gold = dfs(
                    &grid,
                    &mut vec![vec![false; n]; m],
                    i as i32,
                    j as i32,
                    0,
                    25, // 最多25个格子有黄金
                );
                max_gold = max_gold.max(gold);
            }
        }

        fn dfs(
            grid: &Vec<Vec<i32>>,
            visit: &mut Vec<Vec<bool>>,
            i: i32,
            j: i32,
            collected: u8,
            max_collect: u8,
        ) -> i32 {
            if collected > max_collect {
                // 已开采的格子达到最大值
                return 0;
            }
            if i < 0 || j < 0 {
                return 0;
            }
            let m = grid.len();
            let n = grid[0].len();

            let i = i as usize;
            let j = j as usize;
            if i >= m || j >= n {
                return 0;
            }
            if visit[i][j] {
                return 0;
            }
            if grid[i][j] == 0 {
                return 0;
            }

            visit[i][j] = true;

            let next = vec![(-1, 0), (0, 1), (0, -1), (1, 0)];
            let mut next_gold = 0;
            for (_, (x, y)) in next.iter().enumerate() {
                let next_i = i as i32 + x;
                let next_j = j as i32 + y;
                next_gold =
                    dfs(grid, visit, next_i, next_j, collected + 1, max_collect).max(next_gold);
            }
            // 回溯
            visit[i][j] = false;
            grid[i][j] + next_gold
        }

        max_gold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // 0,6,0
        // 5,8,7
        // 0,9,0
        // 一种收集最多黄金的路线是：9 -> 8 -> 7
        let ans = Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]);
        assert_eq!(ans, 24);
    }

    #[test]
    fn t2() {
        // 1,0,7
        // 2,0,6
        // 3,4,5
        // 0,3,0
        // 9,0,20
        // 一种收集最多黄金的路线是：1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7
        let ans = Solution::get_maximum_gold(vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20],
        ]);
        assert_eq!(ans, 28);
    }
}
