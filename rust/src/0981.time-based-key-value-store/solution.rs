// Created by Olgierd Palasz at 2026/08/11 19:50
// leetgo: dev
// https://leetcode.com/problems/time-based-key-value-store/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::{collections::HashMap, println};
struct TimeMap {
    map: HashMap<String, HashMap<i32, String>>,
    timestamps: HashMap<String, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
            timestamps: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key.clone())
            .or_default()
            .insert(timestamp, value);
        self.timestamps.entry(key).or_default().push(timestamp);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(array) = self.timestamps.get(&key) else {
            return String::default();
        };

        let mut left = 0;
        let mut right = array.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if array[mid] <= timestamp {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        match left {
            0 => String::default(),
            _ => self
                .map
                .get(&key)
                .unwrap()
                .get(&array[left - 1])
                .unwrap()
                .to_string(),
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = TimeMap::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "set" => {
                let method_params = split_array(&params[i])?;
                let key: String = deserialize(&method_params[0])?;
                let value: String = deserialize(&method_params[1])?;
                let timestamp: i32 = deserialize(&method_params[2])?;
                obj.set(key, value, timestamp);
                output.push("null".to_string());
            }
            "get" => {
                let method_params = split_array(&params[i])?;
                let key: String = deserialize(&method_params[0])?;
                let timestamp: i32 = deserialize(&method_params[1])?;
                let ans: String = obj.get(key, timestamp).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
