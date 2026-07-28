// Created by Olgierd Palasz at 2026/07/28 17:01
// leetgo: dev
// https://leetcode.com/problems/longest-mountain-in-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = 1;

        let mut longest: usize = 0;

        while left < right && right < arr.len() {
            if arr[left] < arr[right] {
                let start = left;

                while arr[left] < arr[right] {
                    left += 1;
                    right += 1;
                    if !Self::is_safe(right, arr.len()) {
                        return longest as i32;
                    }
                }

                if arr[left] == arr[right] {
                    left += 1;
                    right += 1;
                    if !Self::is_safe(right, arr.len()) {
                        return longest as i32;
                    }
                    continue;
                }

                while arr[left] > arr[right] {
                    left += 1;
                    right += 1;
                    if !Self::is_safe(right, arr.len()) {
                        return longest.max(right - start) as i32;
                    }
                }
                longest = longest.max(right - start);
            } else {
                left += 1;
                right += 1;
            }
        }

        0.max(longest as i32)
    }

    fn is_safe(i: usize, n: usize) -> bool {
        i < n
    }
}

// @lc code=end

fn main() -> Result<()> {
    let arr: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::longest_mountain(arr);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
