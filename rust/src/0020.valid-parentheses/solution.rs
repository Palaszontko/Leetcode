// Created by Olgierd Palasz at 2026/08/04 20:45
// leetgo: dev
// https://leetcode.com/problems/valid-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());

        for b in s.bytes() {
            match b {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                _ => {
                    if stack.pop() != Some(b) {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_valid(s);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
