// Created by Olgierd Palasz at 2026/07/15 21:18
// leetgo: dev
// https://leetcode.com/problems/top-k-frequent-elements/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashed: HashMap<i32, usize> = HashMap::new();

        for num in nums.into_iter() {
            *hashed.entry(num).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<(usize, i32)> =
            hashed.into_iter().map(|(val, cnt)| (cnt, val)).collect();

        let mut result: Vec<i32> = Vec::with_capacity(k as usize);

        for _ in 0..k {
            result.push(heap.pop().map(|(_, value)| value).unwrap());
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::top_k_frequent(nums, k);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
