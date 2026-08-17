// Created by Olgierd Palasz at 2026/08/13 17:23
// leetgo: dev
// https://leetcode.com/problems/merge-two-sorted-lists/

use std::println;

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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;

        while list1.is_some() && list2.is_some() {
            let from_1 = list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val;

            let node = if from_1 {
                let mut n = list1.take().unwrap();
                list1 = n.next.take();
                n
            } else {
                let mut n = list2.take().unwrap();
                list2 = n.next.take();
                n
            };

            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        *tail = list1.or(list2);

        head
    }
}

// @lc code=end

fn main() -> Result<()> {
    let list1: LinkedList = deserialize(&read_line()?)?;
    let list2: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::merge_two_lists(list1.into(), list2.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
