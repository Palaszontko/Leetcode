// Created by Olgierd Palasz at 2026/09/04 22:17
// leetgo: dev
// https://leetcode.com/problems/k-closest-points-to-origin/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::{collections::BinaryHeap, vec};
#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let distance_a = self.x * self.x + self.y * self.y;
        let distance_b = other.x * other.x + other.y * other.y;
        distance_a.cmp(&distance_b)
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Point> = BinaryHeap::with_capacity(k as usize);

        for point in points {
            match point[..] {
                [x, y] => heap.push(Point { x, y }),
                _ => continue,
            };
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        let mut result: Vec<Vec<i32>> = Vec::new();

        for _ in 0..k {
            let res = heap.pop().unwrap();
            result.push(vec![res.x, res.y]);
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let points: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::k_closest(points, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
