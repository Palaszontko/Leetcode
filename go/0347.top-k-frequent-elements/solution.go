// Created by Olgierd Palasz at 2026/05/06 21:34
// leetgo: dev
// https://leetcode.com/problems/top-k-frequent-elements/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

// Bucket-sort approach (faster) — used as the main solution.
func topKFrequent(nums []int, k int) []int {
	hash := make(map[int]int, k)
	for _, val := range nums {
		hash[val] += 1
	}

	count := make([][]int, len(nums)+1)
	for key, val := range hash {
		count[val] = append(count[val], key)
	}

	final := []int{}
	for i := len(count) - 1; i >= 0; i-- {
		for j := 0; j < len(count[i]); j++ {
			final = append(final, count[i][j])
			if len(final) == k {
				return final
			}
		}
	}
	return nil
}

// First approach (slower) — kept for reference/comparison.
func topKFrequentNaive(nums []int, k int) []int {
	hash := make(map[int]int, k)
	maxx := 0

	for _, val := range nums {
		hash[val] += 1
		maxx = max(maxx, hash[val])
	}

	array := []int{}
	for i := 0; i < k; i++ {
		for key, val := range hash {
			if val == maxx {
				array = append(array, key)
				delete(hash, key)
				maxx = 0
			}
		}
		for _, val := range hash {
			maxx = max(maxx, val)
		}
	}
	return array
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	k := Deserialize[int](ReadLine(stdin))
	ans := topKFrequent(nums, k)

	fmt.Println("\noutput:", Serialize(ans))
}
