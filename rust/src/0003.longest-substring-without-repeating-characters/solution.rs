// Created by Olgierd Palasz at 2026/07/22 22:35
// leetgo: dev
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut seen: HashSet<char> = HashSet::new();
        let mut left = 0;
        let mut right = 1;

        let letters: Vec<char> = s.chars().collect();
        let mut max_size = 1;

        seen.insert(*letters.first().unwrap());

        while left < right && right < s.len() {
            if seen.insert(letters[right]) {
                right += 1;
            } else {
                seen.remove(&letters[right]);

                while letters[left] != letters[right] {
                    seen.remove(&letters[left]);
                    left += 1;
                }

                if left == right - 1 || left == right {
                    seen.clear();
                    seen.insert(letters[left]);
                    left += 1;
                    right += 1;
                } else {
                    left += 1;
                }
            }

            max_size = max_size.max(seen.len());
        }

        max_size as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::length_of_longest_substring(s);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
