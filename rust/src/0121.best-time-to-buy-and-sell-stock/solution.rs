// Created by Olgierd Palasz at 2026/08/03 22:35
// leetgo: dev
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut best_price: i32 = 0;

        for right in 1..prices.len() {
            if prices[left] > prices[right] {
                left = right;
            } else {
                best_price = best_price.max(prices[right] - prices[left]);
            }
        }

        best_price
    }
}

// @lc code=end

fn main() -> Result<()> {
    let prices: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_profit(prices);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
