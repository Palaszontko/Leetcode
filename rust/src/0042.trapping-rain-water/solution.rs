// Created by Olgierd Palasz at 2026/07/25 13:22
// leetgo: dev
// https://leetcode.com/problems/trapping-rain-water/

use std::println;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut max_left: i32 = height[left];
        let mut max_right: i32 = height[right];

        let mut result = 0;

        while left < right {
            if max_left < max_right {
                left += 1;
                max_left = max_left.max(height[left]);
                result += max_left - height[left];
            } else {
                right -= 1;
                max_right = max_right.max(height[right]);
                result += max_right - height[right];
            }
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let height: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::trap(height).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
