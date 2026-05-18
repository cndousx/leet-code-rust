use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [108. 将有序数组转换为二叉搜索树](https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&nums, 0, nums.len() - 1)
    }
    fn dfs(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            if end != nums.len() - 1 {
                return None;
            }
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[start]))));
        }
        let middle = (start + end) / 2;
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(nums[middle]))));
        let rc_mut = root.as_mut()?;
        rc_mut.borrow_mut().left = Self::dfs(nums, start, middle);
        rc_mut.borrow_mut().right = Self::dfs(nums, middle + 1, end);
        root
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let tree = Solution::sorted_array_to_bst(nums);
        let preorder = TreeNode::preorder_traversal_recursive(&tree);
        let inorder = TreeNode::inorder_traversal_recursive(&tree);
        assert_eq!(preorder, vec![0, -3, -10, 5, 9]);
        assert_eq!(inorder, vec![-10, -3, 0, 5, 9]);
    }
}
