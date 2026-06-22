// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/search-a-2d-matrix/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func searchMatrix(matrix [][]int, target int) bool {
	top_row := 0
	bottom_row := len(matrix) - 1
	left_index := 0
	right_index := len(matrix[0]) - 1

	for left_index <= right_index && top_row <= bottom_row {
		mid_row := int((top_row + bottom_row) / 2)

		if matrix[mid_row][0] <= target && target <= matrix[mid_row][len(matrix[0])-1] {
			mid_index := int((left_index + right_index) / 2)

			if matrix[mid_row][mid_index] < target {
				left_index = mid_index + 1
			} else if matrix[mid_row][mid_index] > target {
				right_index = mid_index - 1
			} else {
				return true
			}
		} else if target < matrix[mid_row][left_index] {
			bottom_row = mid_row - 1
		} else if target > matrix[mid_row][right_index] {
			top_row = mid_row + 1
		}
	}
	return false
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	matrix := Deserialize[[][]int](ReadLine(stdin))
	target := Deserialize[int](ReadLine(stdin))
	ans := searchMatrix(matrix, target)

	fmt.Println("\noutput:", Serialize(ans))
}
