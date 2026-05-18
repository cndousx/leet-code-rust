use crate::common::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

///
/// [617. 合并二叉树](https://leetcode.cn/problems/merge-two-binary-trees/)
///
struct Solution;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() && root2.is_none() {
            return None;
        }
        if root1.is_some() && root2.is_some() {
            let rc1 = root1?;
            let borrow1 = rc1.borrow();
            let rc2 = root2?;
            let borrow2 = rc2.borrow();
            let mut node = TreeNode::new(borrow1.val + borrow2.val);
            node.left = Self::merge_trees(borrow1.left.clone(), borrow2.left.clone());
            node.right = Self::merge_trees(borrow1.right.clone(), borrow2.right.clone());
            return Some(Rc::new(RefCell::new(node)));
        }
        if root1.is_some() {
            let rc1 = root1?;
            let borrow1 = rc1.borrow();
            // 创建一个新的节点
            let mut node = TreeNode::new(borrow1.val);
            node.left = Self::merge_trees(borrow1.left.clone(), None);
            node.right = Self::merge_trees(borrow1.right.clone(), None);
            Some(Rc::new(RefCell::new(node)))
        } else {
            let rc2 = root2?;
            let borrow2 = rc2.borrow();
            // 创建一个新的节点
            let mut node = TreeNode::new(borrow2.val);
            node.left = Self::merge_trees(None, borrow2.left.clone());
            node.right = Self::merge_trees(None, borrow2.right.clone());
            Some(Rc::new(RefCell::new(node)))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let preorder1 = vec![1, 3, 5, 2];
        let inorder1 = vec![5, 3, 1, 2];
        let tree1 = TreeNode::build_binary_tree(&preorder1, &inorder1);

        let preorder2 = vec![2, 1, 4, 3, 7];
        let inorder2 = vec![1, 4, 2, 3, 7];
        let tree2 = TreeNode::build_binary_tree(&preorder2, &inorder2);

        let ans = Solution::merge_trees(tree1, tree2);
        let preorder = TreeNode::preorder_traversal_recursive(&ans);
        assert_eq!(preorder, vec![3, 4, 5, 4, 5, 7]);
        let inorder = TreeNode::inorder_traversal_recursive(&ans);
        assert_eq!(inorder, vec![5, 4, 4, 3, 5, 7]);
        let postorder = TreeNode::postorder_traversal_recursive(&ans);
        assert_eq!(postorder, vec![5, 4, 4, 7, 5, 3]);
    }
}
