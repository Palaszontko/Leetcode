// Created by Olgierd Palasz at 2026/09/04 22:17
// leetgo: dev
// https://leetcode.com/problems/k-closest-points-to-origin/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::BinaryHeap;
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    dist: i32,
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            dist: x * x + y * y,
            x,
            y,
        }
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Point> = BinaryHeap::with_capacity(k as usize);

        for point in points {
            match point[..] {
                [x, y] => heap.push(Point::new(x, y)),
                _ => continue,
            };
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.into_vec()
            .into_iter()
            .map(|p| vec![p.x, p.y])
            .collect()
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
