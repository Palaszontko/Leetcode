// Created by Olgierd Palasz at 2026/07/14 21:55
// leetgo: dev
// https://leetcode.com/problems/valid-anagram/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut table: [i32; 26] = [0i32; 26];

        for (&a, &b) in s.as_bytes().iter().zip(t.as_bytes()) {
            table[(a - b'a') as usize] += 1;
            table[(b - b'a') as usize] -= 1;
        }

        table.iter().all(|&v| v == 0)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_anagram(s, t);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
