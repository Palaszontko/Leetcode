// Created by Olgierd Palasz at 2026/08/09 18:04
// leetgo: dev
// https://leetcode.com/problems/binary-search/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as i32;
            }
        }

        -1
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::search(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
