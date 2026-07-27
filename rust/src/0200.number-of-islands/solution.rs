// Created by Olgierd Palasz at 2026/07/26 12:34
// leetgo: dev
// https://leetcode.com/problems/number-of-islands/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashSet;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited: HashSet<(usize, usize)> =
            HashSet::with_capacity(grid.len() * grid[0].len());
        let mut result = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if val.eq(&'0') {
                    continue;
                } else if visited.insert((i, j)) {
                    result += 1;
                    Self::flood_fill(&mut visited, &grid, i, j);
                }
            }
        }

        result
    }

    fn flood_fill(
        visited: &mut HashSet<(usize, usize)>,
        grid: &Vec<Vec<char>>,
        i: usize,
        j: usize,
    ) {
        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        if grid[i][j].eq(&'0') {
            return;
        }

        for direction in DIRECTIONS {
            let new_i = i as i32 + direction.0;
            let new_j = j as i32 + direction.1;
            if Self::within_bounds(new_i, new_j, grid.len(), grid[0].len())
                && grid[new_i as usize][new_j as usize].eq(&'1')
                && visited.insert((new_i as usize, new_j as usize))
            {
                Self::flood_fill(visited, grid, new_i as usize, new_j as usize);
            }
        }
    }

    fn within_bounds(i: i32, j: i32, width: usize, height: usize) -> bool {
        if (0 <= i && i < width as i32) && (0 <= j && j < height as i32) {
            return true;
        }
        false
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let grid: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::num_islands(grid).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
