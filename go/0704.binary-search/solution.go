// Created by Olgierd Palasz at 2026/05/06 21:34
// leetgo: dev
// https://leetcode.com/problems/binary-search/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func search(nums []int, target int) int {
	left := 0
	right := len(nums)

	for left < right {
		mid := int((left + right) / 2)
		if nums[mid] == target {
			return mid
		} else if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid
		}
	}
	return -1
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	target := Deserialize[int](ReadLine(stdin))
	ans := search(nums, target)

	fmt.Println("\noutput:", Serialize(ans))
}
