// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/group-anagrams/

package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func groupAnagrams(strs []string) [][]string {
	hash_map := make(map[string][]string)

	for _, val := range strs {
		tmp := strings.Split(val, "")
		sort.Strings(tmp)
		key := strings.Join(tmp, "")
		hash_map[key] = append(hash_map[key], val)
	}

	result_array := make([][]string, 0)
	for _, val := range hash_map {
		result_array = append(result_array, val)
	}
	return result_array
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	strs := Deserialize[[]string](ReadLine(stdin))
	ans := groupAnagrams(strs)

	fmt.Println("\noutput:", Serialize(ans))
}
