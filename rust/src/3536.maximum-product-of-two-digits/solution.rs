// Created by Olgierd Palasz at 2026/07/25 19:12
// leetgo: dev
// https://leetcode.com/problems/maximum-product-of-two-digits/

use std::println;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut digits: Vec<u8> = Vec::new();

        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }

        digits.sort_unstable();

        (digits[digits.len() - 1] * digits[digits.len() - 2]) as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_product(n);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
