use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [226 翻转二叉树](https://leetcode.cn/problems/invert-binary-tree/?envType=problem-list-v2&envId=binary-tree)
///
struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let rc = root.clone()?;
        let mut borrow = rc.borrow_mut();
        let left = borrow.left.clone();
        let right = borrow.right.clone();
        borrow.left = right;
        borrow.right = left;
        Self::invert_tree(borrow.left.clone());
        Self::invert_tree(borrow.right.clone());
        root
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// # 示例
    /// ```xml
    ///       4
    ///      / \
    ///     2   7
    ///    / \ / \
    ///   1  3 6  9
    ///   👇👇👇👇👇
    ///       4
    ///      / \
    ///     7   2
    ///    / \ /  \
    ///   9  6 3  1
    /// ```
    #[test]
    fn t1() {
        let preorder = vec![4, 2, 1, 3, 7, 6, 9];
        let inorder = vec![1, 2, 3, 4, 6, 7, 9];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::invert_tree(root.clone());
        assert_eq!(
            TreeNode::preorder_traversal_recursive(&ans),
            vec![4, 7, 9, 6, 2, 3, 1]
        );
        assert_eq!(
            TreeNode::inorder_traversal_recursive(&ans),
            vec![9, 7, 6, 4, 3, 2, 1]
        );
    }
}
