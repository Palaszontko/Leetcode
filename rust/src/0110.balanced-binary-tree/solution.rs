// Created by Olgierd Palasz at 2026/08/29 15:51
// leetgo: dev
// https://leetcode.com/problems/balanced-binary-tree/

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut result = true;

        Self::depth(&root, &mut result);

        result
    }

    fn depth(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut bool) -> i32 {
        if let Some(n) = &node {
            let n = n.borrow();

            let left = &n.left;
            let right = &n.right;

            let left_height = Self::depth(left, result);
            let right_height = Self::depth(right, result);

            if left_height.abs_diff(right_height) > 1 {
                *result = false
            }

            1 + left_height.max(right_height)
        } else {
            0
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_balanced(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
