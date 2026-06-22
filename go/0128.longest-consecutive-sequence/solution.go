// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/longest-consecutive-sequence/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func longestConsecutive(nums []int) int {
	hash_map := make(map[int]bool)
	for _, val := range nums {
		hash_map[val] = true
	}

	best_count := 0
	for _, val := range nums {
		if !hash_map[val-1] {
			tmp := val
			count := 1
			for hash_map[tmp+1] {
				tmp += 1
				count += 1
			}
			best_count = max(best_count, count)
		}
		if best_count > len(nums)/2 {
			break
		}
	}
	return best_count
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	ans := longestConsecutive(nums)

	fmt.Println("\noutput:", Serialize(ans))
}
