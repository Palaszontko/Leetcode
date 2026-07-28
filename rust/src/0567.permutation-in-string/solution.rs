// Created by Olgierd Palasz at 2026/07/27 22:52
// leetgo: dev
// https://leetcode.com/problems/permutation-in-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() {
            return false;
        }

        let mut key: [usize; 26] = [0; 26];
        for letter in s1.bytes() {
            key[(letter - b'a') as usize] += 1;
        }

        let mut left = 0;
        let mut right = s1.len() - 1;

        let mut table: [usize; 26] = [0; 26];

        for letter in &s2.as_bytes()[0..=right] {
            table[(letter - b'a') as usize] += 1;
        }

        while right < s2.len() {
            if table == key {
                return true;
            } else {
                table[(s2.as_bytes()[left] - b'a') as usize] -= 1;

                left += 1;
                right += 1;

                if right == s2.len() {
                    break;
                }

                table[(s2.as_bytes()[right] - b'a') as usize] += 1;
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s1: String = deserialize(&read_line()?)?;
    let s2: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::check_inclusion(s1, s2).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
