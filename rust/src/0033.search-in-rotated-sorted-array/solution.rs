// Created by Olgierd Palasz at 2026/08/10 22:34
// leetgo: dev
// https://leetcode.com/problems/search-in-rotated-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                left = mid + 1
            } else {
                right = mid;
            }
        }

        let smallest = left;

        if target <= nums[nums.len() - 1] {
            left = smallest;
            right = nums.len();
        } else {
            left = 0;
            right = smallest;
        }

        while left < right {
            let mid = left + (right - left) / 2;
            if target < nums[mid] {
                right = mid;
            } else if target > nums[mid] {
                left = mid + 1;
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
