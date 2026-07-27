// Created by Olgierd Palasz at 2026/07/25 19:22
// leetgo: dev
// https://leetcode.com/problems/longest-repeating-character-replacement/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes: Vec<char> = s.chars().collect();
        let mut left: usize = 0;
        let mut seen: HashMap<char, usize> = HashMap::new();

        let mut max_size = 0;

        for right in 0..s.len() {
            *seen.entry(bytes[right]).or_insert(0) += 1;

            if seen.values().sum::<usize>() - seen.values().max().unwrap() > k as usize {
                max_size = max_size.max(right - left);
                seen.entry(bytes[left]).and_modify(|x| *x -= 1);
                left += 1;
            }
        }

        max_size.max(seen.values().sum::<usize>()) as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::character_replacement(s, k);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
