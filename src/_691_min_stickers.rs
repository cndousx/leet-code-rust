use std::collections::HashSet;
use std::collections::VecDeque;

///
/// [691. 贴纸拼词](https://leetcode.cn/problems/stickers-to-spell-word/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let target_chars = target.chars().collect::<Vec<_>>();
        if target_chars.len() == 0 {
            return 0;
        }
        let stickers = stickers
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let set = stickers
            .iter()
            .flat_map(|row| row.iter())
            .collect::<HashSet<_>>();
        for ch in &target_chars {
            if !set.contains(ch) {
                return -1;
            }
        }
        let mut visited = HashSet::new();
        visited.insert(target);
        let mut queue = VecDeque::new();
        queue.push_back((target_chars, 0));
        while let Some((remain, count)) = queue.pop_front() {
            for sticker in &stickers {
                let mut remain = remain.clone();
                for ch in sticker {
                    if let Some(index) = (&mut remain).iter().position(|c| *c == *ch) {
                        remain.remove(index);
                    }
                }
                if !visited.insert(remain.iter().collect::<String>()) {
                    continue;
                }
                if remain.is_empty() {
                    return count + 1;
                }
                queue.push_back((remain.clone(), count + 1));
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string_vec(vs: Vec<&str>) -> Vec<String> {
        vs.iter().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    #[test]
    fn t1() {
        // 用两个with 和一个example
        // th、e、h、a、t
        let min = Solution::min_stickers(
            to_string_vec(vec!["with", "example", "science"]),
            "thehat".to_string(),
        );
        assert_eq!(min, 3);
    }

    #[test]
    fn t2() {
        let min = Solution::min_stickers(to_string_vec(vec!["hello", "rust"]), "a".to_string());
        assert_eq!(min, -1);
    }

    #[test]
    fn t3() {
        let min = Solution::min_stickers(
            to_string_vec(vec!["notice", "possible"]),
            "basicbasic".to_string(),
        );
        assert_eq!(min, -1);
    }
}
