// Created by Olgierd Palasz at 2026/08/30 18:04
// leetgo: dev
// https://leetcode.com/problems/count-good-nodes-in-binary-tree/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

use std::collections::VecDeque;
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
use std::println;
use std::rc::Rc;

struct WrappedTreeNode {
    node: Option<Rc<RefCell<TreeNode>>>,
    last_max: i32,
}

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut que: VecDeque<WrappedTreeNode> = VecDeque::new();
        let mut result = 0;

        que.push_back(WrappedTreeNode {
            node: root.clone(),
            last_max: root.unwrap().borrow().val,
        });

        while !que.is_empty() {
            while let Some(tree_node) = que.pop_front() {
                if let Some(node) = tree_node.node {
                    let n = node.borrow();

                    if n.val >= tree_node.last_max {
                        result += 1;
                    }

                    que.push_back(WrappedTreeNode {
                        node: n.right.clone(),
                        last_max: tree_node.last_max.max(n.val),
                    });

                    que.push_back(WrappedTreeNode {
                        node: n.left.clone(),
                        last_max: tree_node.last_max.max(n.val),
                    });
                }
            }
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::good_nodes(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
