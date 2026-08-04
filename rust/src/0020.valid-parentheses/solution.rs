// Created by Olgierd Palasz at 2026/08/04 20:45
// leetgo: dev
// https://leetcode.com/problems/valid-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::VecDeque;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: VecDeque<u8> = VecDeque::with_capacity(s.len());

        for par in s.bytes() {
            if par.eq(&b'(') || par.eq(&b'[') || par.eq(&b'{') {
                stack.push_back(par);
            } else if stack.back().is_some_and(|x| x.eq(&b'(') && par.eq(&b')'))
                || stack.back().is_some_and(|x| x.eq(&b'[') && par.eq(&b']'))
                || stack.back().is_some_and(|x| x.eq(&b'{') && par.eq(&b'}'))
            {
                stack.pop_back();
            } else {
                return false;
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
