// Created by Olgierd Palasz at 2026/09/13 16:50
// leetgo: dev
// https://leetcode.com/problems/top-k-frequent-words/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq)]
struct HeapEntry {
    count: u16,
    word: String,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .count
            .cmp(&self.count)
            .then_with(|| self.word.cmp(&other.word))
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut words_map: HashMap<&str, u16> = HashMap::with_capacity(words.len());

        for word in &words {
            *words_map.entry(word).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<HeapEntry> = BinaryHeap::with_capacity(k as usize);

        for (word, count) in words_map {
            heap.push(HeapEntry {
                count,
                word: word.to_owned(),
            });

            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(|entry| entry.word)
            .collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let words: Vec<String> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::top_k_frequent(words, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
