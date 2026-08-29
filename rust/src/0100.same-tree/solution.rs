// Created by Olgierd Palasz at 2026/08/29 18:07
// leetgo: dev
// https://leetcode.com/problems/same-tree/

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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut result = true;

        Self::depth_double(&p, &q, &mut result);

        result
    }

    fn depth_double(
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
        result: &mut bool,
    ) {
        match (&p, &q) {
            (Some(node1), Some(node2)) => {
                let n1 = node1.borrow();
                let n2 = node2.borrow();

                if n1.val != n2.val {
                    *result = false;
                }

                Self::depth_double(&n1.left, &n2.left, result);
                Self::depth_double(&n1.right, &n2.right, result);
            }
            (None, Some(_)) | (Some(_), None) => *result = false,
            (None, None) => {}
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let p: BinaryTree = deserialize(&read_line()?)?;
    let q: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_same_tree(p.into(), q.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
