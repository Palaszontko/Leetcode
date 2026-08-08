// Created by Olgierd Palasz at 2026/08/08 12:15
// leetgo: dev
// https://leetcode.com/problems/car-fleet/

use std::println;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
struct Car {
    position: i32,
    speed: i32,
}
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<Car> = position
            .into_iter()
            .zip(speed)
            .map(|(position, speed)| Car { position, speed })
            .collect();

        cars.sort_unstable_by_key(|car| std::cmp::Reverse(car.position));

        let mut stack: Vec<f32> = Vec::with_capacity(cars.len());

        for car in cars {
            stack.push((target - car.position) as f32 / car.speed as f32);

            if stack.len() >= 2 && stack.last() <= stack.get(stack.len() - 2) {
                stack.pop();
            }
        }

        stack.len() as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let target: i32 = deserialize(&read_line()?)?;
    let position: Vec<i32> = deserialize(&read_line()?)?;
    let speed: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::car_fleet(target, position, speed).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
