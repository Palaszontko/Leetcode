package searchinrotatedsortedarray

func Search(nums []int, target int) int {
	
	left := 0
	right := len(nums) - 1

	for left <= right {
		mid := int((left + right) / 2) 

		if target == nums[mid] {
			return mid
		}

		if nums[left] <= nums[mid] {
			if target > nums[mid] || target < nums[left] {
				left = mid + 1
			} else {
				right = mid - 1
			}
		} else {
			if target > nums[right] || target < nums[mid] {
				right = mid - 1
			} else {
				left = mid + 1
			}
		}
	}

	return -1
}
