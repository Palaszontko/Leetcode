// Created by Olgierd Palasz at 2026/05/06 21:28
// leetgo: dev
// https://leetcode.com/problems/two-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let target: i32 = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::two_sum(nums, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
