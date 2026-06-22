// Created by Olgierd Palasz at 2026/06/22 20:38
// leetgo: dev
// https://leetcode.com/problems/3sum/

package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func threeSum(nums []int) [][]int {
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})

	results := [][]int{}

	for i, val := range nums {
		if i > 0 && nums[i-1] == nums[i] {
			continue
		}

		if val > 0 {
			break
		}

		l := i + 1
		r := len(nums) - 1

		for l < r {
			if val+nums[l]+nums[r] < 0 {
				l += 1
			} else if val+nums[l]+nums[r] > 0 {
				r -= 1
			} else {
				results = append(results, []int{val, nums[l], nums[r]})
				l += 1
				r -= 1
				for l < r && nums[l] == nums[l-1] {
					l += 1
				}
				for l < r && nums[r] == nums[r+1] {
					r -= 1
				}
			}
		}
	}

	return results
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	ans := threeSum(nums)

	fmt.Println("\noutput:", Serialize(ans))
}
