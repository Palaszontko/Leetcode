// Created by Olgierd Palasz at 2026/06/22 20:38
// leetgo: dev
// https://leetcode.com/problems/valid-sudoku/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func hashCheck(array []byte) bool {
	hash := make(map[byte]bool)

	for _, val := range array {
		if hash[val] && val != 46 { // byte('.') = 46
			return false
		} else {
			hash[val] = true
		}
	}
	return true
}

func isValidSudoku(board [][]byte) bool {
	// rows
	for _, row := range board {
		if !hashCheck(row) {
			return false
		}
	}

	// columns
	for i := 0; i < 9; i++ {
		array := make([]byte, 9)

		for j := 0; j < 9; j++ {
			array[j] = board[j][i]
		}

		if !hashCheck(array) {
			return false
		}
	}

	a := 0
	b := 0

	// 3x3 squares
	for k := 0; k < 9; k++ {
		array := make([]byte, 0, 9)

		for i := 0; i < 3; i++ {
			for j := 0; j < 3; j++ {
				array = append(array, board[a+i][b+j])
			}
		}

		if a == 6 {
			a = 0
			b += 3
		} else {
			a += 3
		}

		if !hashCheck(array) {
			return false
		}
	}

	return true
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	board := Deserialize[[][]byte](ReadLine(stdin))
	ans := isValidSudoku(board)

	fmt.Println("\noutput:", Serialize(ans))
}
