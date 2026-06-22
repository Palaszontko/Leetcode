// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/generate-parentheses/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func generateParenthesis(n int) []string {
	result := []string{}
	backtrack(&result, "", 0, 0, n)
	return result
}

func backtrack(result *[]string, current string, open int, close int, max int) {
	if len(current) == 2*max {
		*result = append(*result, current)
		return
	}
	if open < max {
		backtrack(result, current+"(", open+1, close, max)
	}
	if close < open {
		backtrack(result, current+")", open, close+1, max)
	}
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	n := Deserialize[int](ReadLine(stdin))
	ans := generateParenthesis(n)

	fmt.Println("\noutput:", Serialize(ans))
}
