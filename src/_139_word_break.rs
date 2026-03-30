///
/// [139. 单词拆分](https://leetcode.cn/problems/word-break/)
///
struct Solution;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words = word_dict
            .iter()
            .map(|e| e.as_str())
            .collect::<std::collections::HashSet<_>>();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for j in (0..i).rev() {
                if dp[j] && words.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()],
        ))
    }

    #[test]
    fn t2() {
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ));
    }

    #[test]
    fn t3() {
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        ))
    }
}
