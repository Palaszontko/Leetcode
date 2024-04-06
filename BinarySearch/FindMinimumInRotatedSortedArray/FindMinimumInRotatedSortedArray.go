package findminimuminrotatedsortedarray

func FindMin(nums []int) int {

	l := 0
	r := len(nums) - 1

	for l < r {
		mid := int((l + r) / 2)

		if nums[l] < nums[r] {
			return nums[l]
		}

		if nums[l] > nums[mid] {
			r = mid 
		} else if nums[mid] > nums[r] {
			l = mid 
		}

		if len(nums[l:r+1]) == 2 {
			if nums[l] < nums[r]{
				return nums[l]
			}else {
				return nums[r]
			}
		}

	}
	return nums[r]
}

// 1 2 3 4 5 6 7

// 7 1 2 3 4 5 6

// 6 7 1 2 3 4 5

// 5 6 7 1 2 3 4

// 4 5 6 7 1 2 3

// 3 4 5 6 7 1 2

// 2 3 4 5 6 7 1
// -------------
// 		   6 7 1
// -------------
//             1 