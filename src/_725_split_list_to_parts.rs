use crate::common::linked_list::*;
///
/// [725. 分割链表](https://leetcode.cn/problems/split-linked-list-in-parts/)
///
struct Solution;
impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        if k <= 1 {
            return vec![head];
        }
        let mut head = head;
        let k = k as usize;
        let mut current = &head;
        let mut len = 0;
        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }
        if len <= k {
            let mut ans = vec![];
            let mut current = head;
            while current.is_some() {
                let next = current.as_mut().unwrap().next.take();
                ans.push(current);
                current = next;
            }
            while ans.len() < k {
                ans.push(None);
            }
            return ans;
        }

        //  如果extra 等于0，则每个部分大小相同
        //  如果extra大于0，则部分长度加1，extra减1
        //  任意两部分的长度差距不能超过 1
        //  extra也要分，
        //  10%3 =1
        //  4 3 3
        //  11 %3 =2
        //  4 4 3
        //  12 %3 =0
        //  4 4 4
        //  13%3 =1
        //  5 4 4

        let part = len / k;
        let extra = len % k;
        let mut part_vec = vec![part; k];
        for i in 0..extra {
            part_vec[i] += 1;
        }

        let mut ans = vec![];
        ans.push(head);
        for part_len in part_vec {
            let mut curr_len = 0;
            let mut current = ans.last_mut().unwrap();

            while curr_len < part_len {
                curr_len += 1;

                if curr_len == part_len {
                    let head = current.as_mut().unwrap().next.take();
                    ans.push(head);
                    if ans.len() == k {
                        return ans;
                    }
                    break;
                } else {
                    if current.is_some() {
                        current = &mut current.as_mut().unwrap().next;
                    } else {
                        break;
                    }
                }
            }
        }
        while ans.len() < k {
            ans.push(None);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let head = vec_to_linked_list(&vec![1, 2, 3], false);
        let ans = Solution::split_list_to_parts(head, 5);
        let expect = vec![
            vec_to_linked_list(&vec![1], false),
            vec_to_linked_list(&vec![2], false),
            vec_to_linked_list(&vec![3], false),
            None,
            None,
        ];
        assert_eq!(ans, expect);
    }

    #[test]
    fn t2() {
        let head = vec_to_linked_list(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], false);
        let expect = vec![vec![1, 2, 3, 4], vec![5, 6, 7], vec![8, 9, 10]];
        let mut ans = vec![];
        for ele in &Solution::split_list_to_parts(head, 3) {
            let mut curr = ele;
            let v = linked_list_to_vec(curr);
            ans.push(v);
        }
        assert_eq!(ans, expect);
    }

    #[test]
    fn t3() {
        let head = vec_to_linked_list(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], false);
        let ans = Solution::split_list_to_parts(head, 3);
        let expect = vec![
            vec_to_linked_list(&vec![1, 2, 3, 4], false),
            vec_to_linked_list(&vec![5, 6, 7, 8], false),
            vec_to_linked_list(&vec![9, 10, 11], false),
        ];
        assert_eq!(ans, expect);
    }

    #[test]
    fn t4() {
        let head = vec_to_linked_list(&vec![0], false);
        let ans = Solution::split_list_to_parts(head, 1);
        let expect = vec![vec_to_linked_list(&vec![0], false)];
        assert_eq!(ans, expect);
    }

    #[test]
    fn t5() {
        let head = vec_to_linked_list(&vec![0, 1], false);
        let ans = Solution::split_list_to_parts(head, 1);
        let expect = vec![vec_to_linked_list(&vec![0, 1], false)];
        assert_eq!(ans, expect);
    }
}
