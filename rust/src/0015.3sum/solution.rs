// Created by Olgierd Palasz at 2026/07/23 19:18
// leetgo: dev
// https://leetcode.com/problems/3sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::{collections::HashSet, println};
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut left: usize;
        let mut right: usize;

        let mut result: HashSet<[i32; 3]> = HashSet::new();

        for (i, number) in nums.iter().enumerate() {
            left = i + 1;
            right = nums.len() - 1;

            while left < right {
                if nums[left] + nums[right] + number < 0 {
                    left += 1;
                } else if nums[left] + nums[right] + number > 0 {
                    right -= 1;
                } else {
                    result.insert([nums[left], *number, nums[right]]);
                    left += 1;
                }
            }
        }

        result.into_iter().map(|t| t.to_vec()).collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::three_sum(nums);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
