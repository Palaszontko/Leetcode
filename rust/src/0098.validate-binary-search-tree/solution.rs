// Created by Olgierd Palasz at 2026/08/30 19:29
// leetgo: dev
// https://leetcode.com/problems/validate-binary-search-tree/

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
use std::collections::VecDeque;
use std::println;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut cur = root;

        let mut result_vec: Vec<i32> = Vec::new();

        // in order now iterative
        while !stack.is_empty() || cur.is_some() {
            while let Some(node) = cur {
                cur = node.borrow().left.clone();
                stack.push(node);
            }
            let node = stack.pop().unwrap();

            if result_vec.last().is_some_and(|&x| x >= node.borrow().val) {
                return false;
            }
            result_vec.push(node.borrow().val);
            cur = node.borrow().right.clone()
        }
        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_valid_bst(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
