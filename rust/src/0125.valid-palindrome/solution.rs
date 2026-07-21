// Created by Olgierd Palasz at 2026/07/21 22:46
// leetgo: dev
// https://leetcode.com/problems/valid-palindrome/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let clean: String = s
            .to_lowercase()
            .chars()
            .filter(|x| x.is_ascii_alphanumeric())
            .collect();

        clean == clean.chars().rev().collect::<String>()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_palindrome(s);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
