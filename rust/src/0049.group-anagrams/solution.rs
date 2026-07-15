// Created by Olgierd Palasz at 2026/07/14 22:32
// leetgo: dev
// https://leetcode.com/problems/group-anagrams/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut mapped: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());

        for word in strs.into_iter() {
            let mut word_chars_sorted: Vec<char> = word.chars().collect();

            word_chars_sorted.sort_unstable();

            let sorted_word: String = word_chars_sorted.iter().collect();

            mapped.entry(sorted_word).or_default().push(word);
        }

        mapped.into_values().collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let strs: Vec<String> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<String>> = Solution::group_anagrams(strs).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
