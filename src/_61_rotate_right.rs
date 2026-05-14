use crate::common::linked_list::*;
///
/// [61. 旋转链表](https://leetcode.cn/problems/rotate-list/)
///
struct Solution;
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut curr = &head;
        let mut len = 0;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }
        if len == 0 {
            return head;
        }
        let k = k % len;
        if k == 0 {
            return head;
        }
        let mut head = head;
        let mut find_tail_pre = &mut head;
        let mut pos = 0;
        while pos != len - k - 1 {
            find_tail_pre = &mut find_tail_pre.as_mut().unwrap().next;
            pos += 1;
        }
        let mut new_head = find_tail_pre.as_mut().unwrap().next.take();

        let mut find_tail = &mut new_head;
        while let Some(node) = find_tail {
            if node.next.is_none() {
                node.next = head;
                break;
            }
            find_tail = &mut node.next;
        }
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let head = vec_to_linked_list(&vec![0, 1, 2], false);
        assert_eq!(
            linked_list_to_vec(&Solution::rotate_right(head, 4)),
            vec![2, 0, 1]
        );
    }

    #[test]
    fn t2() {
        let head = vec_to_linked_list(&vec![1, 2, 3, 4, 5], false);
        assert_eq!(
            linked_list_to_vec(&Solution::rotate_right(head, 2)),
            vec![4, 5, 1, 2, 3]
        );
    }
}
