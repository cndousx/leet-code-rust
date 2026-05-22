///
/// [65. 有效数字](https://leetcode.cn/problems/valid-number/)
///
struct Solution;
impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.trim();
        if s.is_empty() {
            return false;
        }

        let mut digit = false;
        let mut dot = false;
        let mut e = false;

        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => {
                    digit = true;
                }
                '+' | '-' => {
                    // 符号只能出现在开头或'e'之后
                    if i != 0 && !matches!(s.chars().nth(i - 1).unwrap(), 'e' | 'E') {
                        return false;
                    }
                }
                '.' => {
                    // 小数点不能出现在'e'之后，且只能有一个小数点
                    if dot || e {
                        return false;
                    }
                    dot = true;
                }
                'e' | 'E' => {
                    // 'e'不能出现两次，且必须紧跟一个数字
                    if e || !digit || i == s.len() - 1 {
                        return false;
                    }
                    e = true;
                    digit = false; // 需要看到"e"后面的另一个数字
                }
                _ => return false,
            }
        }

        digit
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::is_number("0".to_string()), true);
    }
    #[test]
    fn t2() {
        assert_eq!(Solution::is_number("e".to_string()), false);
    }
    #[test]
    fn t3() {
        assert_eq!(Solution::is_number(".".to_string()), false);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::is_number("0.1".to_string()), true);
    }

    #[test]
    fn t5() {
        assert_eq!(Solution::is_number("inf".to_string()), false);
    }

    #[test]
    fn t6() {
        assert_eq!(Solution::is_number("-8115e957".to_string()), true);
    }
}
