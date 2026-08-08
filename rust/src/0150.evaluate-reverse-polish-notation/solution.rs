// Created by Olgierd Palasz at 2026/08/05 16:50
// leetgo: dev
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut values: Vec<i32> = Vec::with_capacity(tokens.len());
        let mut operators: Vec<String> = Vec::with_capacity(tokens.len());
        for token in tokens.into_iter() {
            match token.parse::<i32>() {
                Ok(value) => values.push(value),
                Err(_) => operators.push(token),
            }

            if values.len() >= 2 && !operators.is_empty() {
                let a = values.pop().unwrap();
                let b = values.pop().unwrap();

                let result: Option<i32> = match operators.pop().unwrap().as_str() {
                    "+" => Some(a + b),
                    "-" => Some(b - a),
                    "*" => Some(a * b),
                    "/" => Some(b / a),
                    _ => None,
                };

                values.push(result.unwrap());
            }
        }

        values[0]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let tokens: Vec<String> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::eval_rpn(tokens);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
