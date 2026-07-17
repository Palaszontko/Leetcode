// Created by Olgierd Palasz at 2026/07/17 17:39
// leetgo: dev
// https://leetcode.com/problems/valid-sudoku/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashSet;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 9];

        for i in 0..9 {
            for j in 0..9 {
                let val = board[i][j];

                if val == '.' {
                    continue;
                }

                let box_indx: usize = (i / 3) * 3 + (j / 3);

                if rows[i].contains(&val)
                    || cols[j].contains(&val)
                    || boxes[box_indx].contains(&val)
                {
                    return false;
                }

                rows[i].insert(val);
                cols[j].insert(val);
                boxes[box_indx].insert(val);
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let board: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_valid_sudoku(board);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
