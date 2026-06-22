// Created by Olgierd Palasz at 2026/05/06 21:34
// leetgo: dev
// https://leetcode.com/problems/contains-duplicate/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func containsDuplicate(nums []int) bool {
	Hash := make(map[int]bool)
	for _, i := range nums {
		if Hash[i] {
			return true
		} else {
			Hash[i] = true
		}
	}
	return false
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	ans := containsDuplicate(nums)

	fmt.Println("\noutput:", Serialize(ans))
}
