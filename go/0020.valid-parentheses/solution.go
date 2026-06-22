// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/valid-parentheses/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func isValid(s string) bool {
	stack := []rune{}

	hash_map_open_to_close := map[rune]rune{
		'(': ')',
		'{': '}',
		'[': ']',
	}

	for _, val := range s {
		if val == '(' || val == '{' || val == '[' {
			stack = append(stack, val)
		} else {
			if len(stack) == 0 {
				return false
			}
			lastest_parenthesis := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			if hash_map_open_to_close[lastest_parenthesis] != val {
				return false
			}
		}
	}
	return len(stack) == 0
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	s := Deserialize[string](ReadLine(stdin))
	ans := isValid(s)

	fmt.Println("\noutput:", Serialize(ans))
}
