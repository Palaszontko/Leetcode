package sum

import (
	"sort"
)

func ThreeSum(nums []int) [][]int {

	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})

	results := [][]int{}

	for i, val := range nums {

		if i > 0 && nums[i-1] == nums[i] {
			continue
		}

		if val > 0 {
			break
		}

		l := i + 1
		r := len(nums) - 1

		for l < r {
			if val+nums[l]+nums[r] < 0 {
				l += 1
			} else if val+nums[l]+nums[r] > 0 {
				r -= 1
			} else {
				results = append(results, []int{val, nums[l], nums[r]})
				l += 1
				r -= 1
				for l < r {
					if nums[l-1] == nums[l] {
						l += 1
					}
					if nums[r] == nums[r+1] {
						r -= 1
					}
				}

			}
		}
	}

	return results
}
