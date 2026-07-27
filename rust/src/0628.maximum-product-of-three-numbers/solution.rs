// Created by Olgierd Palasz at 2026/07/26 17:16
// leetgo: dev
// https://leetcode.com/problems/maximum-product-of-three-numbers/

use std::println;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        nums.select_nth_unstable(2);
        let min1 = nums[0];
        let min2 = nums[1];

        nums.select_nth_unstable(n - 3);

        let top3_product = nums[n - 1] * nums[n - 2] * nums[n - 3];

        let absolute_max = *nums[n - 3..].iter().max().unwrap();
        let min2_max_product = min1 * min2 * absolute_max;

        std::cmp::max(top3_product, min2_max_product)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::maximum_product(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
