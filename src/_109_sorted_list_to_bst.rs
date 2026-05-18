use crate::common::binary_tree::TreeNode;
use crate::common::linked_list::ListNode;
use crate::common::linked_list::vec_to_linked_list;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [109. 有序链表转二叉搜索树](https://leetcode.cn/problems/convert-sorted-list-to-binary-search-tree/description/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        let mut nums = vec![];
        let mut curr = &head;
        while let Some(node) = curr {
            nums.push(node.val);
            curr = &node.next;
        }
        Self::dfs(&nums, 0, nums.len() - 1)
    }
    fn dfs(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return if end != nums.len() - 1 {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(nums[start]))))
            };
        }
        let middle = (start + end) / 2;
        let val = nums[middle];
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        let rc = root.as_mut()?;
        rc.borrow_mut().left = Self::dfs(nums, start, middle);
        rc.borrow_mut().right = Self::dfs(nums, middle + 1, end);
        root
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let head = vec_to_linked_list(&nums, false);
        let tree = Solution::sorted_list_to_bst(head);
        let preorder = TreeNode::preorder_traversal_recursive(&tree);
        let inorder = TreeNode::inorder_traversal_recursive(&tree);
        assert_eq!(preorder, vec![0, -3, -10, 5, 9]);
        assert_eq!(inorder, vec![-10, -3, 0, 5, 9]);
    }
}
