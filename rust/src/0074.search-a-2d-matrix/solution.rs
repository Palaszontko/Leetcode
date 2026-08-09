// Created by Olgierd Palasz at 2026/08/09 19:31
// leetgo: dev
// https://leetcode.com/problems/search-a-2d-matrix/

use std::println;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.first().unwrap().len();
        let m = matrix.len();

        let mut left = 0;
        let mut right = m;
        let mut mid = left + (right - left) / 2;

        while left < right {
            mid = left + (right - left) / 2;

            if !(matrix[mid][0] <= target && target <= matrix[mid][n - 1]) {
                if target < matrix[mid][0] {
                    right = mid;
                } else if target > matrix[mid][n - 1] {
                    left = mid + 1;
                }
            } else {
                break;
            }
        }

        if matrix[mid][0] <= target && target <= matrix[mid][n - 1] {
            let mut left_inner = 0;
            let mut right_inner = n;

            while left_inner < right_inner {
                let mid_inner = left_inner + (right_inner - left_inner) / 2;

                if matrix[mid][mid_inner] < target {
                    left_inner = mid_inner + 1;
                } else if matrix[mid][mid_inner] > target {
                    right_inner = mid_inner;
                } else {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::search_matrix(matrix, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
