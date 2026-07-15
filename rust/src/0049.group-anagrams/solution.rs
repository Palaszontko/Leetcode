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
        let mut mapped: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());

        for word in strs.into_iter() {
            let chars_key = Self::count_chars(&word);

            mapped.entry(chars_key).or_default().push(word);
        }

        mapped.into_values().collect()
    }

    fn count_chars(word: &str) -> [u8; 26] {
        let mut table: [u8; 26] = [0u8; 26];

        for letter in word.as_bytes().iter() {
            table[(letter - b'a') as usize] += 1
        }

        table
    }
}
// @lc code=end

fn main() -> Result<()> {
    let strs: Vec<String> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<String>> = Solution::group_anagrams(strs);

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
