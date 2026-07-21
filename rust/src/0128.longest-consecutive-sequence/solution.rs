// Created by Olgierd Palasz at 2026/07/21 19:51
// leetgo: dev
// https://leetcode.com/problems/longest-consecutive-sequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;
// @lc code=begin
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let values: HashSet<i32> = nums.into_iter().collect();
        let mut seen_amount: HashMap<i32, i32> = HashMap::with_capacity(values.len());

        if values.is_empty() {
            return 0;
        }

        let mut max_ever = 1;

        for num in &values {
            // println!("num: {:?}", num);
            if !seen_amount.contains_key(num) {
                let mut count = 1;
                let mut current = *num;

                while values.contains(&(current + 1)) {
                    if let Some(value) = seen_amount.get(&(current + 1)) {
                        count += *value;
                        break;
                    } else {
                        count += 1;
                    }

                    seen_amount.insert(current + 1, 0);
                    current += 1;
                }

                seen_amount.insert(*num, count);

                max_ever = max_ever.max(count);

                // println!("num {} - {:?}", num, seen_amount);
            }
        }

        max_ever
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::longest_consecutive(nums);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
