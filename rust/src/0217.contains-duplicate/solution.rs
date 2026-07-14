// Created by Olgierd Palasz at 2026/07/14 20:34
// leetgo: dev
// https://leetcode.com/problems/contains-duplicate/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen: HashSet<i32> = HashSet::with_capacity(nums.len());
        nums.into_iter().any(|n| !seen.insert(n))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: bool = Solution::contains_duplicate(nums);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
