// Created by Olgierd Palasz at 2026/07/28 19:36
// leetgo: dev
// https://leetcode.com/problems/minimum-window-substring/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }

        let letters: Vec<char> = s.chars().collect();

        let mut map_letter_index: HashMap<char, Vec<usize>> = HashMap::with_capacity(t.len());
        let mut amounts: HashMap<char, usize> = HashMap::with_capacity(t.len());
        let mut wanted: HashMap<char, usize> = HashMap::with_capacity(t.len());

        for (i, letter) in s.chars().enumerate() {
            if t.contains(letter) {
                map_letter_index.entry(letter).or_default().push(i);
            }
        }

        for letter in t.chars() {
            *wanted.entry(letter).or_default() += 1;
        }

        let mut ranges: Vec<usize> = map_letter_index.values().flatten().copied().collect();

        ranges.sort();

        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut found: usize = 0;
        let expected_amount = wanted.len();

        let mut appeared = false;
        let mut best_found = s.len() + 1;
        let mut best_left = 0;
        let mut best_right = 0;
        let mut hit_right = false;

        while left < ranges.len() {
            if !hit_right {
                let current_letter = letters[ranges[right]];

                *amounts.entry(current_letter).or_default() += 1;

                if *amounts.get(&current_letter).unwrap() == *wanted.get(&current_letter).unwrap() {
                    found += 1;
                }
            }
            while found == expected_amount {
                appeared = true;
                if ranges[right] - ranges[left] + 1 < best_found
                    && ranges[right] - ranges[left] + 1 >= t.len()
                {
                    best_found = ranges[right] - ranges[left] + 1;
                    best_left = ranges[left];
                    best_right = ranges[right];
                }
                *amounts.entry(letters[ranges[left]]).or_default() -= 1;

                if *amounts.get(&letters[ranges[left]]).unwrap()
                    < *wanted.get(&letters[ranges[left]]).unwrap()
                {
                    found -= 1;
                }

                left += 1;
            }

            if right + 1 < ranges.len() {
                right += 1;
            } else {
                hit_right = true;
                left += 1;
            }
        }

        if appeared {
            letters[best_left..=best_right].iter().collect()
        } else {
            "".to_string()
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: String = Solution::min_window(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
