// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/container-with-most-water/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func maxArea(height []int) int {
	l := 0
	r := len(height) - 1
	best := 0

	for l < r {
		area := min(height[l], height[r]) * (r - l)
		if height[l] < height[r] {
			l += 1
		} else {
			r -= 1
		}
		if area > best {
			best = area
		}
	}
	return best
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	height := Deserialize[[]int](ReadLine(stdin))
	ans := maxArea(height)

	fmt.Println("\noutput:", Serialize(ans))
}
