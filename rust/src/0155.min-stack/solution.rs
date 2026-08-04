// Created by Olgierd Palasz at 2026/08/04 21:20
// leetgo: dev
// https://leetcode.com/problems/min-stack/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

struct Data {
    value: i32,
    min: i32,
}

struct MinStack {
    stack: Vec<Data>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::<Data>::new(),
        }
    }

    fn push(&mut self, value: i32) {
        if self.stack.is_empty() {
            self.stack.push(Data { value, min: value });
        } else if value < Self::get_min(self) {
            self.stack.push(Data { value, min: value });
        } else {
            self.stack.push(Data {
                value,
                min: Self::get_min(self),
            });
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().value
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().min
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
