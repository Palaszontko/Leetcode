// Created by Olgierd Palasz at 2026/07/21 23:04
// leetgo: dev
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::ops::Add;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            if numbers[left] + numbers[right] < target {
                left += 1;
            } else if numbers[left] + numbers[right] > target {
                right -= 1;
            } else {
                return vec![left.add(1_usize) as i32, right.add(1_usize) as i32];
            }
        }

        vec![]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let numbers: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::two_sum(numbers, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
