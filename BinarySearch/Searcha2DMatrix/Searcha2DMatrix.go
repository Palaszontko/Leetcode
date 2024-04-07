package searcha2dmatrix

func SearchMatrix2(matrix [][]int, target int) bool {
	// top and bottom row
	top_row := 0
	bottom_row := len(matrix) - 1

	// left and right column
	left_index := 0
	right_index := len(matrix[0]) - 1

	for left_index <= right_index && top_row <= bottom_row {
		mid_row := int((top_row + bottom_row) / 2)

		// we find the correct row
		if matrix[mid_row][0] <= target && target <= matrix[mid_row][len(matrix[0])-1] {
			mid_index := int((left_index + right_index) / 2)

			if matrix[mid_row][mid_index] < target {
				left_index = mid_index + 1
			} else if matrix[mid_row][mid_index] > target {
				right_index = mid_index - 1
			} else {
				return true
			}

			// target is "above" mid row
		} else if target < matrix[mid_row][left_index] {
			bottom_row = mid_row - 1
			// target is "under" mid row
		} else if target > matrix[mid_row][right_index] {
			top_row = mid_row + 1
		}
	}

	return false
}

// (x) - target

// | - left index
// 1  2  3  (4)   5  -- top row
// 6  7  8  9  10
// 11 12 13 14 15  -- mid row
// 16 17 18 19 20
// 21 22 23 24 25  -- bottom row
// 				|  -- right index
