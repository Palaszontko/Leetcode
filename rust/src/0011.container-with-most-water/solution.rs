// Created by Olgierd Palasz at 2026/07/25 08:30
// leetgo: dev
// https://leetcode.com/problems/container-with-most-water/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut result = 0;

        while left < right {
            let area = height[left].min(height[right]) * (right - left) as i32;
            result = result.max(area);

            if height[left] < height[right] {
                left += 1;
            } else if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let height: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_area(height).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
