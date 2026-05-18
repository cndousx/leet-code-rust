use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
///
/// [LCR 144. 翻转二叉树](https://leetcode.cn/problems/er-cha-shu-de-jing-xiang-lcof/)
///
struct Solution;
impl Solution {
    pub fn flip_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let rc = root.clone()?;
        let mut x = rc.borrow_mut();
        let left = x.left.clone();
        let right = x.right.clone();
        x.left = right;
        x.right = left;
        Self::flip_tree(x.left.clone());
        Self::flip_tree(x.right.clone());
        root
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// # 示例
    /// ```xml
    ///       3
    ///      / \
    ///     9   20
    ///    /    / \
    ///   6    15  7
    ///       /
    ///      1
    ///   👇👇👇👇👇
    ///       3
    ///      / \
    ///    20   9
    ///   / \    \
    ///   7  15   6
    ///       \
    ///        1
    /// ```
    #[test]
    fn t1() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let ans = Solution::flip_tree(root.clone());
        assert_eq!(
            TreeNode::preorder_traversal_recursive(&ans),
            vec![3, 20, 7, 15, 1, 9, 6]
        );
        assert_eq!(
            TreeNode::inorder_traversal_recursive(&ans),
            vec![7, 20, 15, 1, 3, 9, 6]
        );
    }
}
