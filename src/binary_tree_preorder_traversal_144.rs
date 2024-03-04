/*
 * @lc app=leetcode.cn id=144 lang=rust
 * @lcpr version=30117
 *
 * [144] 二叉树的前序遍历
 */


// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
// Definition for a binary tree node.

use std::borrow::Borrow;
use std::iter;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    //迭代法
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut target=vec![];
        if root.is_none(){
            return target
        }
        let mut stack =vec![root.unwrap()];
        while let Some(node) = stack.pop() {
            target.push(node.as_ref().borrow().val);
            if let Some(right)=node.as_ref().borrow().right.clone(){
                stack.push(right);
            }
            if let Some(left)=node.as_ref().borrow().left.clone(){
                stack.push(left);
            }
            drop(node);
        }
        target
    }

}
// @lc code=end
struct Solution;

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
      right: None
    }
  }
}

/*
// @lcpr case=start
// [1,null,2,3]\n
// @lcpr case=end

// @lcpr case=start
// []\n
// @lcpr case=end

// @lcpr case=start
// [1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2]\n
// @lcpr case=end

// @lcpr case=start
// [1,null,2]\n
// @lcpr case=end

 */

