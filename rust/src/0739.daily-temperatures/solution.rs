// Created by Olgierd Palasz at 2026/08/05 21:26
// leetgo: dev
// https://leetcode.com/problems/daily-temperatures/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::VecDeque;
struct Entry {
    value: i32,
    index: usize,
}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: VecDeque<Entry> = VecDeque::with_capacity(temperatures.len());
        let mut result: Vec<i32> = vec![0; temperatures.len()];

        for (i, temperature) in temperatures.iter().enumerate() {
            if stack.is_empty() {
                stack.push_back(Entry {
                    value: *temperature,
                    index: i,
                });
                continue;
            }

            while !stack.is_empty() && stack.back().unwrap().value < *temperature {
                if let Some(entry) = stack.pop_back() {
                    result[entry.index] = (i - entry.index) as i32;
                }
            }

            stack.push_back(Entry {
                value: *temperature,
                index: i,
            });

            // if stack.back().unwrap().value < *temperature {
            //     result[stack.front().unwrap().index] = i - stack.front().unwrap().index;
            //     stack.pop_front();
            // }
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let temperatures: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::daily_temperatures(temperatures);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
