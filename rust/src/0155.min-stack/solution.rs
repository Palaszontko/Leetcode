// Created by Olgierd Palasz at 2026/08/04 21:20
// leetgo: dev
// https://leetcode.com/problems/min-stack/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

struct MinStack {
    stack: Vec<i32>,
    min_vals: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_vals: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        if self.stack.is_empty() {
            self.stack.push(value);
            self.min_vals.push(value);
        } else {
            self.stack.push(value);

            if value < Self::get_min(self) {
                self.min_vals.push(value);
            } else {
                self.min_vals.push(self.get_min());
            }
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_vals.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_vals.last().unwrap()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = MinStack::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "push" => {
                let method_params = split_array(&params[i])?;
                let value: i32 = deserialize(&method_params[0])?;
                obj.push(value);
                output.push("null".to_string());
            }
            "pop" => {
                obj.pop();
                output.push("null".to_string());
            }
            "top" => {
                let ans: i32 = obj.top().into();
                output.push(serialize(ans)?);
            }
            "getMin" => {
                let ans: i32 = obj.get_min().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
