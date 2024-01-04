package TwoSum

func TwoSum(nums []int, target int) []int {

	hash := make(map[int]int, len(nums))

	for i, val := range nums {

		if _, ok := hash[val]; ok && hash[val] != i {
			return []int{hash[val], i}
		} else {
			hash[target-val] = i
		}

	}

	return nil
}
