package ValidSudoku

func Hash_check(array []byte) bool {
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

func IsValidSudoku(board [][]byte) bool {
	//board length -> 9

	//rows
	for _, row := range board {
		if !Hash_check(row) {
			return false
		}
	}

	//columns
	for i := 0; i < 9; i++ {
		array := make([]byte, 9)

		for j := 0; j < 9; j++ {
			array[j] = board[j][i]
		}

		if !Hash_check(array) {
			return false
		}
	}

	a := 0
	b := 0

	// 3x3 squares
	for k := 0; k < 9; k++ {
		array := make([]byte, 9)

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

		if !Hash_check(array) {
			return false
		}

	}

	return true
}
