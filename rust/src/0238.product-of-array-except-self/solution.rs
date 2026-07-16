// Created by Olgierd Palasz at 2026/07/16 08:04
// leetgo: dev
// https://leetcode.com/problems/product-of-array-except-self/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let prefix_factor: Vec<i32> = nums
            .iter()
            .scan(1, |state, val| {
                *state *= *val;
                Some(*state)
            })
            .collect();

        let suffix_factor: Vec<i32> = nums
            .iter()
            .rev()
            .scan(1, |state, val| {
                *state *= *val;
                Some(*state)
            })
            .collect::<Vec<i32>>()
            .into_iter()
            .rev()
            .collect();

        let mut result: Vec<i32> = vec![1; nums.len()];

        *result.first_mut().unwrap() = suffix_factor[1];
        *result.last_mut().unwrap() = prefix_factor[nums.len() - 2];

        for i in 1..(nums.len() - 1) {
            result[i] = prefix_factor[i - 1] * suffix_factor[i + 1]
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::product_except_self(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
