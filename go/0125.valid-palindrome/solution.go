// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/valid-palindrome/

package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func isPalindrome(s string) bool {
	array := strings.ToLower(s)

	i := 0
	j := len(array) - 1

	for i < j {
		if (array[i] < 'a' || array[i] > 'z') && (array[i] < '0' || array[i] > '9') {
			i += 1
			continue
		}
		if (array[j] < 'a' || array[j] > 'z') && (array[j] < '0' || array[j] > '9') {
			j -= 1
			continue
		}
		if array[i] == array[j] {
			i += 1
			j -= 1
		} else {
			return false
		}
	}
	return true
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	s := Deserialize[string](ReadLine(stdin))
	ans := isPalindrome(s)

	fmt.Println("\noutput:", Serialize(ans))
}
