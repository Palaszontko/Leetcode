// Created by Olgierd Palasz at 2026/08/09 20:36
// leetgo: dev
// https://leetcode.com/problems/koko-eating-bananas/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut start = 1;
        let mut end = *piles.iter().max().unwrap();

        while start < end {
            let mid = start + (end - start) / 2;

            let amount: i32 = piles.iter().map(|&x| (x + mid - 1) / mid).sum();

            if amount > h {
                start = mid + 1;
            } else {
                end = mid;
            }
        }

        start
    }
}

// @lc code=end
fn main() -> Result<()> {
    let piles: Vec<i32> = deserialize(&read_line()?)?;
    let h: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::min_eating_speed(piles, h).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
