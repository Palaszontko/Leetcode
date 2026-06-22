// Created by Olgierd Palasz at 2026/05/06 21:34
// leetgo: dev
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func twoSum(numbers []int, target int) []int {
	i := 0
	j := len(numbers) - 1

	for numbers[i]+numbers[j] != target {
		if numbers[i]+numbers[j] > target {
			j -= 1
		} else {
			i += 1
		}
	}
	return []int{i + 1, j + 1}
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	numbers := Deserialize[[]int](ReadLine(stdin))
	target := Deserialize[int](ReadLine(stdin))
	ans := twoSum(numbers, target)

	fmt.Println("\noutput:", Serialize(ans))
}
