// Created by Olgierd Palasz at 2026/08/10 20:13
// leetgo: dev
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        let mut result = nums[0];

        while left < right {
            let mid = left + (right - left) / 2;

            result = result.min(nums[mid]);
            result = result.min(nums[left]);
            result = result.min(nums[right]);

            if nums[left] <= nums[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::find_min(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
