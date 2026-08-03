// Created by Olgierd Palasz at 2026/07/31 20:20
// leetgo: dev
// https://leetcode.com/problems/sliding-window-maximum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(k as usize);

        let mut que: VecDeque<usize> = VecDeque::with_capacity(nums.len());

        for i in 0..nums.len() {
            if !que.is_empty()
                && i.checked_sub(k as usize).is_some()
                && *que.front().unwrap() == (i - k as usize)
            {
                que.pop_front();
            }

            while !que.is_empty() && nums[*que.back().unwrap()] < nums[i] {
                que.pop_back();
            }

            que.push_back(i);

            if i + 1 >= k as usize {
                result.push(nums[*que.front().unwrap()]);
            }
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::max_sliding_window(nums, k);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
