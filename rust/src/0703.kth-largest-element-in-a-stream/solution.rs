// Created by Olgierd Palasz at 2026/09/03 19:43
// leetgo: dev
// https://leetcode.com/problems/kth-largest-element-in-a-stream/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::{cmp::Reverse, collections::BinaryHeap};
struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    max_size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k as usize + 1);
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        KthLargest {
            heap,
            max_size: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.max_size {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0 // strange xd
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let k: i32 = deserialize(&constructor_params[0])?;
    let nums: Vec<i32> = deserialize(&constructor_params[1])?;
    let nums_size: i32 = deserialize(&constructor_params[2])?;
    #[allow(unused_mut)]
    let mut obj = KthLargest::new(k, nums, nums_size);

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "add" => {
                let method_params = split_array(&params[i])?;
                let val: i32 = deserialize(&method_params[0])?;
                let ans: i32 = obj.add(val).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
