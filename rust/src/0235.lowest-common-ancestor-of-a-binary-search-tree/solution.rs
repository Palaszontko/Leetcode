// Created by Olgierd Palasz at 2026/08/29 20:39
// leetgo: dev
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_value = p.unwrap().borrow().val;
        let q_value = q.unwrap().borrow().val;

        let mut cur = root;

        while let Some(node) = cur {
            let val = node.borrow().val;

            if p_value < val && q_value < val {
                cur = node.borrow().left.clone();
            } else if p_value > val && q_value > val {
                cur = node.borrow().right.clone();
            } else {
                return Some(node);
            }
        }
        None
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let p: i32 = deserialize(&read_line()?)?;
    let q: i32 = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::lowest_common_ancestor(root.into(), p, q).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
