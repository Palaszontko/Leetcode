// Created by Olgierd Palasz at 2026/08/29 13:53
// leetgo: dev
// https://leetcode.com/problems/maximum-depth-of-binary-tree/

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find_depth(&root)
    }

    fn find_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let n = n.borrow();
            1 + Self::find_depth(&n.left).max(Self::find_depth(&n.right))
        } else {
            0
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_depth(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
