// Created by Olgierd Palasz at 2026/08/19 16:40
// leetgo: dev
// https://leetcode.com/problems/cinema-seat-allocation/

use std::println;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_number_of_families(n: i32, mut reserved_seats: Vec<Vec<i32>>) -> i32 {
        reserved_seats.sort_by_key(|x| *x.first().unwrap());

        let mut start_row = *reserved_seats.first().unwrap().first().unwrap();

        let mask_mid: u16 = 0b0001111000;
        let mask_left: u16 = 0b0111100000;
        let mask_right: u16 = 0b0000011110;
        let mask_whole: u16 = mask_left | mask_right;

        let mut mask: u16 = 0;

        let mut result = 0;

        let mut changes = 0;

        for row in reserved_seats {
            if *row.first().unwrap() == start_row {
                mask |= 1 << (10 - row.get(1).unwrap());
            } else {
                if mask != 0 {
                    if (mask ^ mask_whole) & mask_whole == mask_whole {
                        result += 2;
                    } else if ((mask ^ mask_left) & mask_left == mask_left)
                        || ((mask ^ mask_right) & mask_right == mask_right)
                        || ((mask ^ mask_mid) & mask_mid == mask_mid)
                    {
                        result += 1;
                    }
                }

                start_row = *row.first().unwrap();
                mask = 0;
                mask |= 1 << (10 - row.get(1).unwrap());
                changes += 1;
            }
        }

        if mask != 0 {
            if (mask ^ mask_whole) & mask_whole == mask_whole {
                result += 2;
            } else if ((mask ^ mask_left) & mask_left == mask_left)
                || ((mask ^ mask_right) & mask_right == mask_right)
                || ((mask ^ mask_mid) & mask_mid == mask_mid)
            {
                result += 1;
            }
        }

        result += (n - (changes + 1)) * 2;

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let reserved_seats: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_number_of_families(n, reserved_seats).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
