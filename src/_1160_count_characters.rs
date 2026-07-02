use std::collections::HashMap;

///
/// [1160. 拼写单词](https://leetcode.cn/problems/find-words-that-can-be-formed-by-characters/)
///
struct Solution;
impl Solution {
    fn char_frequency(s: String) -> HashMap<char, i32> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut frequency = HashMap::new();
        for c in chars {
            *frequency.entry(c).or_insert(0) += 1;
        }
        frequency
    }

    ///
    /// 给定一个字符串数组 words 和一个字符串 chars。
    ///
    /// 如果字符串可以由 chars 中的字符组成（每个字符在 每个 words 中只能使用一次），则认为它是好的。
    ///
    /// 返回 words 中所有好的字符串的长度之和。
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_map = Self::char_frequency(chars);

        let mut ans = 0;
        for word in words {
            let mut allow = true;
            let wrod_len = word.len();
            let chs = Self::char_frequency(word);
            for (_, (ch, fre)) in chs.iter().enumerate() {
                match chars_map.get(ch) {
                    Some(count) => {
                        if count < fre {
                            allow = false;
                            break;
                        }
                    }
                    None => {
                        allow = false;
                        break;
                    }
                }
            }
            if allow {
                ans += wrod_len;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::str_util::to_string_vec;

    #[test]
    fn t1() {
        // 每个字符在每个words中只能使用一次
        // cat和hat中的t是两个word所以可以使用多次
        // 如果有字符串 aaat，则不行，因为atach里只有两个a
        // 可以形成字符串 "cat" 和 "hat"，所以答案是 3 + 3 = 6。
        assert_eq!(
            Solution::count_characters(
                to_string_vec(vec!["cat", "bt", "hat", "tree"]),
                "atach".to_string()
            ),
            6
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::count_characters(
                to_string_vec(vec!["hello", "world", "leetcode"]),
                "welldonehoneyr".to_string()
            ),
            10
        );
    }
}
