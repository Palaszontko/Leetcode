// Created by Olgierd Palasz at 2026/07/16 08:04
// leetgo: dev
// https://leetcode.com/problems/product-of-array-except-self/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![1; nums.len()];

        let mut prefix_mul: i32 = 1;
        for i in 0..nums.len() {
            result[i] = prefix_mul;
            prefix_mul *= nums[i];
        }

        let mut suffix_mul: i32 = 1;
        for i in (0..nums.len()).rev() {
            result[i] *= suffix_mul;
            suffix_mul *= nums[i];
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::product_except_self(nums);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
