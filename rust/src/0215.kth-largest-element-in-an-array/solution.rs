// Created by Olgierd Palasz at 2026/09/05 01:10
// leetgo: dev
// https://leetcode.com/problems/kth-largest-element-in-an-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k as usize);

        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.pop().unwrap().0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::find_kth_largest(nums, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
