use std::collections::HashMap;
///
/// [383. 赎金信](https://leetcode.cn/problems/ransom-note/)
///
struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.is_empty() {
            return true;
        }
        let ransom_note = Self::char_frequency(ransom_note);
        let magazine = Self::char_frequency(magazine);

        for (_, (ch, frequency)) in ransom_note.iter().enumerate() {
            match magazine.get(ch) {
                Some(count) => {
                    if count < frequency {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }

    /// 统计每个字符出现的次数
    fn char_frequency(s: String) -> HashMap<char, i32> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut map = HashMap::new();
        for c in chars {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn t2() {
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
    }

    #[test]
    fn t3() {
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }
}
