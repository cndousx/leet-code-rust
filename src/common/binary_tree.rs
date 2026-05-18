use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    ///
    /// 根据前缀和中缀构建二叉树
    ///
    /// # 参数
    /// - `preorder`: 前缀,数组中不可出现重复元素
    /// - `inorder`: 中缀,数组中不可出现重复元素
    ///
    /// # 返回值
    ///
    /// [`TreeNode`]根节点
    ///
    /// # 示例
    /// ```
    ///  //     3
    ///  //    / \
    ///  //   9   20
    ///  //  /    / \
    ///  // 6    15  7
    ///  //     /
    ///  //    1
    ///  use leet_code_rust::common::binary_tree::TreeNode;
    ///  let preorder = vec![3, 9, 6, 20, 15, 1, 7];
    ///  let inorder = vec![6, 9, 3, 1, 15, 20, 7];
    ///  let root = TreeNode::build_binary_tree(&preorder, &inorder);
    /// ```
    pub fn build_binary_tree(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        Self::build_verify(preorder, inorder);
        let n = preorder.len();
        Self::build_by_preorder_inorder(preorder, inorder, 0, n - 1, 0, n - 1)
    }

    fn build_verify(preorder: &Vec<i32>, inorder: &Vec<i32>) {
        if preorder.len() != inorder.len() {
            panic!()
        }
        Self::verify_duplicate(preorder);
        Self::verify_duplicate(inorder);
        let preorder: HashSet<i32> = HashSet::from_iter(preorder.iter().cloned());
        let inorder: HashSet<i32> = HashSet::from_iter(inorder.iter().cloned());
        if preorder != inorder {
            // 元素要相同
            panic!()
        }
    }

    /// 校验数组中是否出现重复的元素
    fn verify_duplicate(vec: &Vec<i32>) {
        let mut set = HashSet::new();
        for x in vec {
            if !set.insert(x) {
                panic!("duplicate：{}", x);
            }
        }
    }

    fn build_by_preorder_inorder(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
        pre_left: usize,
        pre_right: usize,
        in_left: usize,
        in_right: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_left > pre_right || in_left > in_right {
            return None;
        }
        // 从当前需要处理的 前序数组从获取，根结点值=> 前序数组左边界就是根结点值
        let mut node = Self::new(preorder[pre_left]);

        // 从中序数组中，从左边界向右找，当前根节点在中序数组中的下标
        let mut in_root_idx = in_left;
        while in_root_idx <= in_right && inorder[in_root_idx] != preorder[pre_left] {
            in_root_idx += 1;
        }
        // 根据根结点在中序数组中的下标 和 当前中序数组的左边界 => 得到 左子树长度
        let mut sub_left_len = in_root_idx - in_left;
        node.right = Self::build_by_preorder_inorder(
            preorder,
            inorder,
            pre_left + sub_left_len + 1,
            pre_right,
            in_root_idx + 1,
            in_right,
        );
        node.left = if in_root_idx >= 1 {
            // in_root_idx - 1 usize 可能溢出(负数)
            Self::build_by_preorder_inorder(
                preorder,
                inorder,
                pre_left + 1,
                pre_left + sub_left_len,
                in_left,
                in_root_idx - 1,
            )
        } else {
            None
        };
        Some(Rc::new(RefCell::new(node)))
    }

    /// 根据 顺序储存 构建
    ///
    /// 最后一层的叶子节点的孩子可不写
    /// # 参数
    /// - sequential: 下标从1开始
    ///
    /// # 返回值
    ///
    /// [`TreeNode`]根节点
    ///
    /// # 示例
    /// ```
    ///  //     3
    ///  //    / \
    ///  //   9   20
    ///  //  /    / \
    ///  // 6    15  7
    ///  //     /
    ///  //    1
    ///  use leet_code_rust::common::binary_tree::TreeNode;
    ///  let sequential = vec![
    ///            None,
    ///            Some(3),
    ///            Some(9), Some(20),
    ///            Some(6), None, Some(15), Some(7),
    ///            None, None, None, None, Some(1), None, None, None, None,
    ///        ];
    ///        let root = TreeNode::build_by_sequential_storage(&sequential);
    ///        assert_eq!(
    ///           TreeNode::postorder_traversal_iter(root.clone()),
    ///           vec![6, 9, 1, 15, 7, 20, 3]
    ///       );
    ///
    /// ```
    pub fn build_by_sequential_storage(
        sequential: &Vec<Option<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_by_sequential_storage_recursive(sequential, 1)
    }
    fn build_by_sequential_storage_recursive(
        sequential: &Vec<Option<i32>>,
        index: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if index >= sequential.len() {
            return None;
        }
        // 当前节点的node
        match sequential[index] {
            None => None,
            Some(val) => {
                let mut node = TreeNode::new(val);
                node.left = Self::build_by_sequential_storage_recursive(sequential, 2 * index);
                node.right = Self::build_by_sequential_storage_recursive(sequential, 2 * index + 1);
                Some(Rc::new(RefCell::new(node)))
            }
        }
    }
    ///
    /// 构建完全二叉树
    ///
    /// 下标从1开始
    pub fn build_complete_tree(sequential: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let sequential = sequential
            .iter()
            .map(|x| Some(x.clone()))
            .collect::<Vec<Option<i32>>>();
        Self::build_by_sequential_storage(&sequential)
    }

    ///
    /// 构建二叉树
    ///
    /// [`i32::MIN`]当成空节点
    ///
    /// 下标从1开始
    pub fn build_tree_by_sequential_storage(
        sequential: &Vec<i32>,
        min_as_empty: bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let sequential = sequential
            .iter()
            .map(|x| {
                let v = x.clone();
                if !min_as_empty {
                    return Some(v);
                }
                if v == i32::MIN { None } else { Some(v) }
            })
            .collect::<Vec<Option<i32>>>();
        Self::build_by_sequential_storage(&sequential)
    }
    ///
    /// 构建二叉树
    ///
    /// [`i32::MIN`]当成空节点
    ///
    pub fn build_tree_by_two_sequential(
        two_sequential: Vec<Vec<i32>>,
        min_as_empty: bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // 二维数组
        let mut sequential = two_sequential.into_iter().flatten().collect::<Vec<i32>>();
        sequential.insert(0, i32::MIN);
        Self::build_tree_by_sequential_storage(&sequential, min_as_empty)
    }
    /// 前序遍历二叉树(递归)
    pub fn preorder_traversal_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut preorder = vec![root.borrow().val];
                let left = Self::preorder_traversal_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        preorder.push(x);
                    }
                }
                let right = Self::preorder_traversal_recursive(&root.borrow().right);
                if !right.is_empty() {
                    for x in right {
                        preorder.push(x);
                    }
                }
                preorder
            }
        }
    }
    /// 中序遍历二叉树(递归)
    pub fn inorder_traversal_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut inorder = vec![];
                let left = Self::inorder_traversal_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        inorder.push(x);
                    }
                }
                inorder.push(root.borrow().val);
                let right = Self::inorder_traversal_recursive(&root.borrow().right);
                if !right.is_empty() {
                    for x in right {
                        inorder.push(x);
                    }
                }
                inorder
            }
        }
    }

    /// 后序遍历二叉树(递归)
    pub fn postorder_traversal_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut postorder = vec![];
                let left = Self::postorder_traversal_recursive(&root.borrow().left);
                if !left.is_empty() {
                    for x in left {
                        postorder.push(x);
                    }
                }
                let right = Self::postorder_traversal_recursive(&root.borrow().right);
                if !right.is_empty() {
                    for x in right {
                        postorder.push(x);
                    }
                }
                postorder.push(root.borrow().val);
                postorder
            }
        }
    }
    ///  层次遍历(迭代)
    ///
    /// 按层返回，这里提高层序遍历代码可读性
    pub fn level_order_traversal_iter(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut level_order_traversal = vec![];
        // 下一层
        let mut next_level_node = VecDeque::new();
        // 当前层
        let mut curr_level_node = VecDeque::new();
        curr_level_node.push_back(root.clone());
        let mut curr_level_traversal = vec![];
        loop {
            match curr_level_node.pop_front() {
                Some(node) => match node {
                    Some(rc) => {
                        curr_level_traversal.push(rc.borrow().val);
                        next_level_node.push_back(rc.borrow().left.clone());
                        next_level_node.push_back(rc.borrow().right.clone());
                    }
                    None => {}
                },
                None => {}
            }
            if curr_level_node.is_empty() && !next_level_node.is_empty() {
                while let Some(rc) = next_level_node.pop_front() {
                    curr_level_node.push_back(rc);
                }
                if !curr_level_traversal.is_empty() {
                    level_order_traversal.push(curr_level_traversal);
                    curr_level_traversal = vec![];
                }

                continue;
            }
            if curr_level_node.is_empty() && next_level_node.is_empty() {
                if !curr_level_traversal.is_empty() {
                    level_order_traversal.push(curr_level_traversal);
                }
                return level_order_traversal;
            }
        }
    }

    /// 前序遍历二叉树(非递归) 迭代
    pub fn preorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 非递归和递归思路一样都是利用栈实现
        // rust语言提供的递归是利用栈实现的，直接使用递归函数，节省了开发者的编码且易于理解。
        // 非递归是用队列自定义栈，模拟rust语言的递归遍历过程
        let mut preorder = vec![];
        // Vec 更适合栈操作，性能优于 VecDeque
        let mut stack = Vec::new();

        // 使用栈初始化根节点
        if let Some(node) = root {
            stack.push(node);
        }

        while let Some(current) = stack.pop() {
            // 访问当前节点
            preorder.push(current.borrow().val);

            // 先压右子树，再压左子树，确保左子树优先被访问
            if let Some(right) = current.borrow().right.clone() {
                stack.push(right);
            }
            if let Some(left) = current.borrow().left.clone() {
                stack.push(left);
            }
        }

        preorder
    }

    /// 后序遍历二叉树(非递归) 迭代
    pub fn postorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        // Vec 更适合栈操作，性能优于 VecDeque
        let mut stack = Vec::new();

        // 使用栈初始化根节点
        if let Some(node) = root {
            stack.push(node);
        }

        while let Some(current) = stack.pop() {
            // 访问当前节点
            result.push(current.borrow().val);

            // 先压左子树，再压右子树，最后再反转结果
            if let Some(left) = current.borrow().left.clone() {
                stack.push(left);
            }

            if let Some(right) = current.borrow().right.clone() {
                stack.push(right);
            }
        }
        result.reverse();
        result
    }

    /// 中序遍历二叉树(非递归) 迭代
    pub fn inorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut inorder = vec![];
        // vec 替代 VecDeque,Vec 更适合后进先出的需求
        let mut stack = Vec::new();
        let mut curr = root;
        while curr.is_some() || !stack.is_empty() {
            // 尽可能往左走，并把路径上的节点压入栈中
            while let Some(curr_rc) = curr {
                stack.push(curr_rc.clone());
                curr = curr_rc.borrow().left.clone();
            }
            // 弹出栈顶节点，访问它并转向右子树
            if let Some(stack_pop) = stack.pop() {
                inorder.push(stack_pop.borrow().val);
                curr = stack_pop.borrow().right.clone();
            }
        }
        inorder
    }
    /// 构建二叉搜索树
    ///
    /// ```
    /// use leet_code_rust::common::binary_tree::TreeNode;
    /// let nums = vec![-10, -3, 0, 5, 9];
    /// let tree = TreeNode::build_bst(&nums);
    /// let preorder = TreeNode::preorder_traversal_recursive(&tree);
    /// let inorder = TreeNode::inorder_traversal_recursive(&tree);
    /// assert_eq!(preorder, vec![0, -3, -10, 5, 9]);
    /// assert_eq!(inorder, vec![-10, -3, 0, 5, 9]);
    /// ```
    ///
    pub fn build_bst(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_bst_dfs(nums, 0, nums.len() - 1)
    }
    fn build_bst_dfs(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            if end != nums.len() - 1 {
                return None;
            }
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[start]))));
        }
        let middle = (start + end) / 2;
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(nums[middle]))));
        let rc_mut = root.as_mut()?;
        rc_mut.borrow_mut().left = Self::build_bst_dfs(nums, start, middle);
        rc_mut.borrow_mut().right = Self::build_bst_dfs(nums, middle + 1, end);
        root
    }
    /// 深度遍历寻找值和参数相同的节点，返回第一个()
    pub fn find_first_val(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => {
                return None;
            }
            Some(rc) => {
                let borrow = rc.borrow();
                if borrow.val == val {
                    return Some(rc.clone());
                }

                let left = Self::find_first_val(borrow.left.clone(), val);
                if left.is_some() {
                    return left;
                }
                Self::find_first_val(borrow.right.clone(), val)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::preorder_traversal_recursive(&root), preorder);
        assert_eq!(TreeNode::inorder_traversal_recursive(&root), inorder);
        assert_eq!(
            TreeNode::postorder_traversal_recursive(&root),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
        assert_eq!(
            TreeNode::level_order_traversal_iter(&root),
            vec![vec![3], vec![9, 20], vec![6, 15, 7], vec![1]],
        );
    }

    #[test]
    fn t2() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::preorder_traversal_iter(root.clone()), preorder);
    }

    #[test]
    fn t3() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(
            TreeNode::postorder_traversal_iter(root.clone()),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
    }

    #[test]
    fn t4() {
        let sequential = vec![
            None,
            Some(3),
            Some(9),
            Some(20),
            Some(6),
            None,
            Some(15),
            Some(7),
            None,
            None,
            None,
            None,
            Some(1),
            None,
            None,
            None,
            None,
        ];
        let root = TreeNode::build_by_sequential_storage(&sequential);
        assert_eq!(
            TreeNode::postorder_traversal_iter(root.clone()),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
    }

    #[test]
    fn t5() {
        let sequential = vec![
            i32::MIN,
            3,
            9,
            20,
            6,
            i32::MIN,
            15,
            7,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            1,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
        ];
        let sequential = sequential
            .iter()
            .map(|x| if *x == i32::MIN { None } else { Some(*x) })
            .collect::<Vec<Option<i32>>>();
        let root = TreeNode::build_by_sequential_storage(&sequential);
        assert_eq!(
            TreeNode::postorder_traversal_iter(root.clone()),
            vec![6, 9, 1, 15, 7, 20, 3]
        );
    }

    #[test]
    fn t6() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::inorder_traversal_iter(root.clone()), inorder);
    }

    #[test]
    fn t7() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::inorder_traversal_iter(root.clone()), inorder);
    }

    #[test]
    fn t8() {
        let preorder = vec![5, 4, 1, 2, 6];
        let inorder = vec![1, 4, 2, 5, 6];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        assert_eq!(TreeNode::inorder_traversal_iter(root.clone()), inorder);
    }

    #[test]
    fn t9() {
        let nums = vec![-10, -3, 0, 5, 9];
        let tree = TreeNode::build_bst(&nums);
        let preorder = TreeNode::preorder_traversal_recursive(&tree);
        let inorder = TreeNode::inorder_traversal_recursive(&tree);
        assert_eq!(preorder, vec![0, -3, -10, 5, 9]);
        assert_eq!(inorder, vec![-10, -3, 0, 5, 9]);
    }
    #[test]
    fn t10() {
        let preorder = vec![3, 9, 6, 20, 15, 1, 7];
        let inorder = vec![6, 9, 3, 1, 15, 20, 7];
        let root = TreeNode::build_binary_tree(&preorder, &inorder);
        let option = TreeNode::find_first_val(root.clone(), 9);
        assert_eq!(option.unwrap().borrow().val, 9);
    }
}
