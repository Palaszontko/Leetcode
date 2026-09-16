// Created by Olgierd Palasz at 2026/09/16 19:07
// leetgo: dev
// https://leetcode.com/problems/subsets/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(nums.len().pow(2));
        let mut current: Vec<i32> = Vec::new();

        Self::req(&nums, &mut result, &mut current, 0);

        result
    }

    fn req(nums: &Vec<i32>, result: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, index: usize) {
        if index == nums.len() {
            result.push(current.clone());
            return;
        }

        Self::req(nums, result, current, index + 1);

        current.push(nums[index]);

        Self::req(nums, result, current, index + 1);

        current.pop();
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::subsets(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
