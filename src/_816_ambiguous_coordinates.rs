///
/// [816. 模糊坐标](https://leetcode.cn/problems/ambiguous-coordinates/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    /// 生成字符串 s 的所有合法数字表示（整数或小数）
    fn valid_numbers(s: &str) -> Vec<String> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }
        let bytes = s.as_bytes();
        let mut res = Vec::new();

        // 1. 整数形式：只有一位或首位不为 '0'
        if n == 1 || bytes[0] != b'0' {
            res.push(s.to_string());
        }

        // 2. 小数形式：枚举小数点位置 i（1..n）
        for i in 1..n {
            let int_part = &s[..i];
            let frac_part = &s[i..];

            // 整数部分合法：长度为1或首位非0
            if int_part.len() > 1 && int_part.starts_with('0') {
                continue;
            }
            // 小数部分合法：末尾不能为0
            if frac_part.ends_with('0') {
                continue;
            }
            res.push(format!("{}.{}", int_part, frac_part));
        }

        res
    }

    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        // 去掉外层括号
        let inner = s.trim_start_matches('(').trim_end_matches(')');
        let n = inner.len();
        let mut ans = Vec::new();

        // 枚举逗号位置（分割点），从1到n-1
        for split in 1..n {
            let left = &inner[..split];
            let right = &inner[split..];

            let left_nums = Self::valid_numbers(left);
            let right_nums = Self::valid_numbers(right);

            if left_nums.is_empty() || right_nums.is_empty() {
                continue;
            }

            for l in &left_nums {
                for r in &right_nums {
                    ans.push(format!("({}, {})", l, r));
                }
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn t1() {
        let ans = Solution::ambiguous_coordinates("(123)".to_string());
        assert_eq!(
            HashSet::from_iter(ans.into_iter()),
            hashset![
                "(1, 23)".to_string(),
                "(12, 3)".to_string(),
                "(1.2, 3)".to_string(),
                "(1, 2.3)".to_string()
            ]
        );
    }

    #[test]
    fn t2() {
        let ans = Solution::ambiguous_coordinates("(00011)".to_string());
        assert_eq!(
            HashSet::from_iter(ans.into_iter()),
            hashset!["(0.001, 1)".to_string(), "(0, 0.011)".to_string()]
        );
    }

    #[test]
    fn t3() {
        let ans = Solution::ambiguous_coordinates("(0123)".to_string());
        assert_eq!(
            HashSet::from_iter(ans.into_iter()),
            hashset![
                "(0, 123)".to_string(),
                "(0, 12.3)".to_string(),
                "(0, 1.23)".to_string(),
                "(0.1, 23)".to_string(),
                "(0.1, 2.3)".to_string(),
                "(0.12, 3)".to_string(),
            ]
        );
    }

    #[test]
    fn t4() {
        let ans = Solution::ambiguous_coordinates("(100)".to_string());
        assert_eq!(
            HashSet::from_iter(ans.into_iter()),
            hashset!["(10, 0)".to_string()]
        );
    }
}
