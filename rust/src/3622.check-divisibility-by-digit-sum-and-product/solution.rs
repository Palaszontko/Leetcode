// Created by Olgierd Palasz at 2026/08/22 10:13
// leetgo: dev
// https://leetcode.com/problems/check-divisibility-by-digit-sum-and-product/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn check_divisibility(mut n: i32) -> bool {
        let copy = n;
        let mut sum = 0;
        let mut product = 1;

        while n != 0 {
            sum += n % 10;
            product *= n % 10;
            n /= 10;
        }

        copy % (sum + product) == 0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::check_divisibility(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
