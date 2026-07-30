use std::collections::HashSet;
use std::collections::VecDeque;

///
/// [773. 滑动谜题](https://leetcode.cn/problems/sliding-puzzle/?envType=problem-list-v2&envId=backtracking)
///
struct Solution;
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let target = "123450";
        let mut start = board
            .iter()
            .map(|f| f.iter().map(|i| i.to_string()).collect::<String>())
            .collect::<String>();
        if start == target {
            return 0;
        }
        // 预计算：一维索引 i 对应的相邻位置（基于2x3网格）
        //   0 1 2
        //   3 4 5
        //  末尾填充-1是为了对其数组长度，如果用vec则需要运行时申请内存
        let neighbors = [
            [1, 3, -1], // 0 的邻居
            [0, 2, 4],  // 1 的邻居
            [1, 5, -1], // 2 的邻居
            [0, 4, -1], // 3 的邻居
            [1, 3, 5],  // 4 的邻居
            [2, 4, -1], // 5 的邻居
        ];
        let mut queue = VecDeque::new();
        queue.push_back(start.clone());
        let mut visited = HashSet::new();
        visited.insert(start);

        let mut steps = 0;
        while !queue.is_empty() {
            //进入新一层时，步数才+1
            steps += 1;

            // 记录当前层的节点数
            let level_size = queue.len();

            for _ in 0..level_size {
                let state = queue.pop_front().unwrap();
                let zero_index = state
                    .chars()
                    .collect::<Vec<_>>()
                    .iter()
                    .position(|c| *c == '0')
                    .unwrap();
                for neighbor in &neighbors[zero_index] {
                    if *neighbor >= 0 {
                        let mut lst = state.chars().collect::<Vec<_>>();
                        lst.swap(zero_index, *neighbor as usize);
                        let ns = lst.iter().collect::<String>();
                        if ns == target {
                            return steps;
                        }

                        if !visited.contains(&ns) {
                            visited.insert(ns.clone());
                            queue.push_back(ns);
                        }
                    }
                }
            }
        }

        -1
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]),
            1
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]),
            -1
        );
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
            5
        );
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 5, 0]]),
            0
        );
    }
}
