///
/// [44. 通配符匹配](https://leetcode.cn/problems/wildcard-matching/)
///
struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let m = s.len();
        let p = p.chars().collect::<Vec<char>>();
        let n = p.len();

        // dp[i][j] 表示字符串 s[0..i] 与模式 p[0..j] 是否匹配
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        // 处理模式开头连续的 * 情况，例如模式 "*","**" 等可匹配空字符串
        for j in 1..=n {
            if p[j - 1] == '*' {
                dp[0][j] = dp[0][j - 1];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                if p[j - 1] == s[i - 1] || p[j - 1] == '?' {
                    // 字符相同或模式为 ?，则取决于之前的匹配状态
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p[j - 1] == '*' {
                    // 模式为 *，可以匹配空(即dp[i][j-1])或者匹配一个或多个字符(即dp[i-1][j])
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                }
                // 其他情况保持默认值 false
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let s = String::from("aa");
        let p = String::from("a");
        assert_eq!(Solution::is_match(s, p), false);
    }

    #[test]
    fn t2() {
        let s = String::from("aa");
        let p = String::from("a*");
        assert_eq!(Solution::is_match(s, p), true);
    }
}
