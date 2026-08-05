// Created by Olgierd Palasz at 2026/08/05 21:05
// leetgo: dev
// https://leetcode.com/problems/generate-parentheses/

use std::unreachable;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        let mut result: Vec<String> = Vec::new();

        Self::generate(&mut stack, &mut result, 0, 0, n as usize);

        result
    }

    fn generate(
        stack: &mut Vec<String>,
        result: &mut Vec<String>,
        open_n: usize,
        closed_n: usize,
        n: usize,
    ) {
        if open_n == n && closed_n == n {
            result.push(stack.concat());
            return;
        }

        if open_n < n {
            stack.push("(".to_string());
            Self::generate(stack, result, open_n + 1, closed_n, n);
            stack.pop();
        }

        if closed_n < open_n {
            stack.push(")".to_string());
            Self::generate(stack, result, open_n, closed_n + 1, n);
            stack.pop();
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::generate_parenthesis(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
