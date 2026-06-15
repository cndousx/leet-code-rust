///
/// [221. 最大正方形](https://leetcode.cn/problems/maximal-square/?envType=problem-list-v2&envId=dynamic-programming)
///
struct Solution;
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];
        if matrix[0][0] == '1' {
            dp[0][0] = 1;
        }
        let mut max_side_len = dp[0][0];
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }
                dp[i][j] = if matrix[i][j] == '1' {
                    1 + if i == 0 || j == 0 {
                        // 同一行或者同一列不可能形成新的正方形
                        0
                    } else {
                        dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
                    }
                } else {
                    0
                };
                if dp[i][j] > max_side_len {
                    max_side_len = dp[i][j];
                }
            }
        }

        max_side_len * max_side_len
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ]);
        assert_eq!(ans, 4);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::maximal_square(vec![vec!['0']]), 0);
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::maximal_square(vec![vec!['0', '1'], vec!['1', '0']]),
            1
        );
    }
}
