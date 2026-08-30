// Created by Olgierd Palasz at 2026/08/29 23:52
// leetgo: dev
// https://leetcode.com/problems/binary-tree-level-order-traversal/

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
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut que: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

        que.push_back(root.clone());

        while !que.is_empty() {
            let mut que2: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
            let mut valeus: Vec<i32> = Vec::new();

            while let Some(node) = que.pop_front() {
                match node {
                    Some(n) => {
                        let n = n.borrow();
                        valeus.push(n.val);

                        que2.push_back(n.left.clone());
                        que2.push_back(n.right.clone());
                    }
                    None => continue,
                }
            }
            if !valeus.is_empty() {
                result.push(valeus);
            }

            que.append(&mut que2);
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::level_order(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
