// Created by Olgierd Palasz at 2026/08/31 19:24
// leetgo: dev
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder, &inorder)
    }

    fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let (&root_val, rest_pre) = preorder.split_first()?;

        let mid = inorder.iter().position(|&x| x == root_val).unwrap();

        let (left_in, right_in) = inorder.split_at(mid);
        let right_in = &right_in[1..];

        let (left_pre, right_pre) = rest_pre.split_at(mid);

        let left = Self::build(left_pre, left_in);
        let right = Self::build(right_pre, right_in);

        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left,
            right,
        })))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let preorder: Vec<i32> = deserialize(&read_line()?)?;
    let inorder: Vec<i32> = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::build_tree(preorder, inorder).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
