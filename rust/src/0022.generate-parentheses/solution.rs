// Created by Olgierd Palasz at 2026/08/05 21:05
// leetgo: dev
// https://leetcode.com/problems/generate-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::VecDeque;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut res: Vec<String> = vec![];
        let mut q: VecDeque<(String, usize, usize)> = VecDeque::new();
        q.push_back(("(".to_string(), 1, 0));

        while let Some((s, open, close)) = q.pop_front() {
            if close == open && open == n {
                res.push(s);
                continue;
            }

            if close < n && (close + 1) <= open {
                q.push_back((format!("{s})"), open, close + 1));
            }

            if open < n {
                q.push_back((format!("{s}("), open + 1, close));
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::generate_parenthesis(n);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
