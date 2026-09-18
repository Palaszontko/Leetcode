// Created by Olgierd Palasz at 2026/09/16 21:49
// leetgo: dev
// https://leetcode.com/problems/combination-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::{HashSet, VecDeque};

struct Path {
    value: i32,
    path: Vec<i32>,
}

impl Path {
    pub fn new(value: i32, path: Vec<i32>) -> Self {
        Path { value, path }
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let start = Path {
            value: target,
            path: Vec::new(),
        };

        let mut result: HashSet<Vec<i32>> = HashSet::new();

        let mut que: VecDeque<Path> = VecDeque::new();

        que.push_front(start);

        while let Some(current) = que.pop_front() {
            if current.value == 0 {
                let mut result_path = current.path.clone();
                result_path.sort();
                result.insert(result_path);
                continue;
            }

            for n in &candidates {
                if current.value - n >= 0 {
                    let mut new_path = current.path.clone();
                    new_path.push(*n);
                    que.push_back(Path::new(current.value - n, new_path));
                }
            }
        }

        result.into_iter().collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let candidates: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::combination_sum(candidates, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
