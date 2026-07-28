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
        let mut grid = grid;

        let m = grid.len();
        let n = grid[0].len();
        let mut max_gold = 0;
        for i in 0..m {
            for j in 0..n {
                let gold = dfs(&mut grid, i, j);
                max_gold = max_gold.max(gold);
            }
        }

        #[inline]
        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            let m = grid.len();
            let n = grid[0].len();

            if i >= m || j >= n {
                return 0;
            }
            let gold = grid[i][j];
            if gold == 0 {
                return 0;
            }
            // 标记已经开采
            grid[i][j] = 0;

            let mut best_next_gold = 0;
            // 用数组替换vec，提高运行速度
            // 数组: 栈上（小）/ 内联，零开销，无分配
            // Vec<T>: 堆上分配，有堆分配 + 可能的 realloc
            let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (x, y) in &directions {
                if let Ok(ni) = usize::try_from(i as i32 + x)
                    && let Ok(nj) = usize::try_from(j as i32 + y)
                {
                    if ni < m && nj < n && grid[ni][nj] > 0 {
                        best_next_gold = dfs(grid, ni, nj).max(best_next_gold);
                    }
                }
            }
            // 回溯
            grid[i][j] = gold;
            gold + best_next_gold
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
