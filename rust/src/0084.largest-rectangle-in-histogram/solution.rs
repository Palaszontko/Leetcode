// Created by Olgierd Palasz at 2026/08/08 23:23
// leetgo: dev
// https://leetcode.com/problems/largest-rectangle-in-histogram/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
struct Entry {
    index: usize,
    value: i32,
}
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let size = heights.len();
        let mut stack: Vec<Entry> = Vec::with_capacity(size);

        let mut best_rec: i32 = heights[0];

        for (index, value) in heights.into_iter().enumerate() {
            if stack.is_empty() {
                stack.push(Entry { index, value });
            } else {
                _ = 1;

                if value >= stack.last().unwrap().value {
                    stack.push(Entry { index, value });
                } else {
                    let mut last_idx: Option<usize> = None;

                    while let Some(top) = stack.last() {
                        if value < top.value {
                            let area = top.value * (index - top.index) as i32;
                            best_rec = best_rec.max(top.value);
                            best_rec = best_rec.max(area);
                            last_idx = Some(top.index);
                            stack.pop();
                        } else {
                            break;
                        }
                    }

                    if let Some(last) = last_idx {
                        stack.push(Entry { index: last, value });
                    } else {
                        stack.push(Entry { index, value });
                    }
                }
            }
        }

        for entry in stack.into_iter() {
            best_rec = best_rec.max(entry.value * (size - entry.index) as i32);
        }

        best_rec
    }
}

// @lc code=end

fn main() -> Result<()> {
    let heights: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::largest_rectangle_area(heights).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
