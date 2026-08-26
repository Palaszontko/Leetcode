// Created by Olgierd Palasz at 2026/08/26 16:20
// leetgo: dev
// https://leetcode.com/problems/shortest-and-lexicographically-smallest-beautiful-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let number = s.as_bytes();
        let mut best_size = s.len();
        let mut best_val: u128 = u128::MAX;
        let mut one_amount = 0;
        let mut left = 0;
        let mut right = 0;

        let mut found = false;

        while right < s.len() {
            if number[right] == b'1' {
                one_amount += 1;
            }

            while one_amount == k {
                found = true;
                if (right - left + 1) <= best_size {
                    let x = number[left..=right]
                        .iter()
                        .fold(0u128, |acc, &val| (acc << 1) | ((val - b'0') as u128));

                    best_val = best_val.min(x);
                    best_size = best_size.min(right - left + 1)
                }
                if number[left] == b'1' {
                    one_amount -= 1;
                }
                left += 1;

                if left == s.len() - 1 {
                    break;
                }
            }

            right += 1;
        }

        if found {
            format!("{:b}", best_val)
        } else {
            String::default()
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: String = Solution::shortest_beautiful_substring(s, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
