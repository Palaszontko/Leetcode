// Created by Olgierd Palasz at 2026/09/03 21:11
// leetgo: dev
// https://leetcode.com/problems/last-stone-weight/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);

        while heap.len() > 1 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();
            heap.push(y - x);
        }

        heap.pop().unwrap()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let stones: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::last_stone_weight(stones).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
