// Created by Olgierd Palasz at 2026/05/06 21:28
// leetgo: dev
// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (i, n) in nums.into_iter().enumerate() {
            let complement = target - n;

            if let Some(&j) = seen.get(&complement) {
                return vec![j as i32, i as i32];
            }

            seen.insert(n, i);
        }

        Vec::new()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::two_sum(nums, target);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
