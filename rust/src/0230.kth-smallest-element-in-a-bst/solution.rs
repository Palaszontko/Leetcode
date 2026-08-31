// Created by Olgierd Palasz at 2026/08/31 18:59
// leetgo: dev
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/

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
use std::println;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut calls = 0;
        Self::in_order(&root, &mut calls, k).unwrap()
    }

    fn in_order(
        node: &Option<Rc<RefCell<TreeNode>>>,
        calls: &mut i32,
        expected: i32,
    ) -> Option<i32> {
        if let Some(n) = node {
            let n = n.borrow();

            if n.left.is_some()
                && let Some(x) = Self::in_order(&n.left, calls, expected)
            {
                return Some(x);
            }

            *calls += 1;
            if *calls == expected {
                return Some(n.val);
            }

            if n.right.is_some() {
                return Self::in_order(&n.right, calls, expected);
            }
        }
        None
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::kth_smallest(root.into(), k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
