// Created by Olgierd Palasz at 2026/08/18 21:41
// leetgo: dev
// https://leetcode.com/problems/find-the-largest-almost-missing-integer/

use std::collections::BTreeMap;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();

        nums.iter().for_each(|&x| *map.entry(x).or_default() += 1);

        if k == 1 {
            let result = map
                .iter()
                .rev()
                .find(|(_, val)| **val == 1)
                .map(|(key, _)| key);

            match result {
                Some(_) => return *result.unwrap(),
                None => return -1,
            }
        }

        if k as usize == nums.len() {
            return nums.into_iter().max().unwrap();
        }

        let mut result: i32 = -1;

        if *map.get(&nums[0]).unwrap() == 1 {
            result = result.max(nums[0])
        }

        if *map.get(&nums[nums.len() - 1]).unwrap() == 1 {
            result = result.max(nums[nums.len() - 1])
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::largest_integer(nums, k);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
