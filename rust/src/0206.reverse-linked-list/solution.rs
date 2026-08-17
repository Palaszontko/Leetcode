// Created by Olgierd Palasz at 2026/08/12 20:39
// leetgo: dev
// https://leetcode.com/problems/reverse-linked-list/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::reverse_list(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
