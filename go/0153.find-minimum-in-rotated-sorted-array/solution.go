// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func findMin(nums []int) int {
	l := 0
	r := len(nums) - 1

	for l < r {
		mid := int((l + r) / 2)
		if nums[l] < nums[r] {
			return nums[l]
		}
		if nums[l] > nums[mid] {
			r = mid
		} else if nums[mid] > nums[r] {
			l = mid
		}
		if len(nums[l:r+1]) == 2 {
			return nums[r]
		}
	}
	return nums[r]
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	ans := findMin(nums)

	fmt.Println("\noutput:", Serialize(ans))
}
