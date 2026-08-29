// Created by Olgierd Palasz at 2026/08/29 18:34
// leetgo: dev
// https://leetcode.com/problems/subtree-of-another-tree/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut result = false;

        Self::depth(&root, &sub_root, &mut result);

        result
    }

    fn depth(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
        result: &mut bool,
    ) {
        if let (Some(node), Some(sub_node)) = (root, sub_root) {
            let n = node.borrow();
            let n_sub = sub_node.borrow();
            if n.val == n_sub.val {
                let mut result_sub = true;
                Self::compare(root, sub_root, &mut result_sub);
                if result_sub {
                    *result = true;
                    return;
                }
            }
            Self::depth(&n.left, sub_root, result);
            Self::depth(&n.right, sub_root, result);
        }
    }

    fn compare(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
        result: &mut bool,
    ) {
        match (root, sub_root) {
            (Some(node), Some(sub_node)) => {
                let n = node.borrow();
                let n_sub = sub_node.borrow();

                if n.val != n_sub.val {
                    *result = false;
                } else {
                    Self::compare(&n.left, &n_sub.left, result);
                    Self::compare(&n.right, &n_sub.right, result);
                }
            }
            (None, Some(_)) | (Some(_), None) => {
                *result = false;
            }
            (None, None) => {}
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let sub_root: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_subtree(root.into(), sub_root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
