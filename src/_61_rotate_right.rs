use crate::common::linked_list::*;

///
/// [61. 旋转链表](https://leetcode.cn/problems/rotate-list/)
///
struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 计算链表长度
        let mut current = &head;
        let mut length = 0;
        while let Some(node) = current {
            length += 1;
            current = &node.next;
        }

        // 边界条件处理
        if length == 0 || k % length == 0 {
            return head;
        }

        // 实际需要向右移动的位置数
        let actual_rotation = k % length;
        // 分割点位置
        let split_position = length - actual_rotation;

        let mut head_node = head;
        let mut prev_to_split_point = &mut head_node;

        // 找到分割点前一个节点
        for _ in 0..split_position - 1 {
            prev_to_split_point = &mut prev_to_split_point.as_mut()?.next;
        }

        // 新头节点为分割点后的节点
        let mut new_head = prev_to_split_point.as_mut()?.next.take();

        // 找到新头节点的尾部，并连接原头部
        let mut tail_node = &mut new_head;
        loop {
            if let Some(node) = tail_node {
                if node.next.is_none() {
                    node.next = head_node;
                    break;
                }
                tail_node = &mut node.next;
            } else {
                break;
            }
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
