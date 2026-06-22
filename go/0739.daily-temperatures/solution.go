// Created by Olgierd Palasz at 2026/06/22 20:38
// leetgo: dev
// https://leetcode.com/problems/daily-temperatures/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func dailyTemperatures(temperatures []int) []int {
	result := make([]int, len(temperatures))
	stack := []int{} // indices of unresolved temperatures, decreasing

	for i, t := range temperatures {
		for len(stack) > 0 && temperatures[stack[len(stack)-1]] < t {
			j := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			result[j] = i - j
		}
		stack = append(stack, i)
	}

	return result
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	temperatures := Deserialize[[]int](ReadLine(stdin))
	ans := dailyTemperatures(temperatures)

	fmt.Println("\noutput:", Serialize(ans))
}
