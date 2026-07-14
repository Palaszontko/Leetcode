// Created by Olgierd Palasz at 2026/06/23 18:18
// leetgo: dev
// https://leetcode.com/problems/daily-temperatures/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        
    }
}

// @lc code=end

fn main() -> Result<()> {
	let temperatures: Vec<i32> = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::daily_temperatures(temperatures).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
