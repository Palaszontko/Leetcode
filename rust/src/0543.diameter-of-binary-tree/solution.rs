// Created by Olgierd Palasz at 2026/08/29 14:20
// leetgo: dev
// https://leetcode.com/problems/diameter-of-binary-tree/

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
use std::fmt::Alignment::Right;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut best = 0;

        Self::find_depth(&root, &mut best);

        best
    }

    fn find_depth(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
        if let Some(n) = node {
            let n = n.borrow();
            let left_best_size = Self::find_depth(&n.left, best);
            let right_best_size = Self::find_depth(&n.right, best);
            *best = (*best).max(left_best_size + right_best_size);
            1 + left_best_size.max(right_best_size)
        } else {
            0
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::diameter_of_binary_tree(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
