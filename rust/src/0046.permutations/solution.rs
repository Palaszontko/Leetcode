// Created by Olgierd Palasz at 2026/09/18 16:53
// leetgo: dev
// https://leetcode.com/problems/permutations/

use std::vec;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        Self::back(vec![], &mut result, nums);
        result
    }

    fn back(current: Vec<i32>, result: &mut Vec<Vec<i32>>, nums: Vec<i32>) {
        if nums.is_empty() {
            result.push(current.clone());
            return;
        }

        for i in 0..nums.len() {
            let mut current_copy = current.clone();
            let mut nums_copy = nums.clone();
            current_copy.push(nums[i]);
            nums_copy.remove(i);
            Self::back(current_copy, result, nums_copy);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::permute(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
