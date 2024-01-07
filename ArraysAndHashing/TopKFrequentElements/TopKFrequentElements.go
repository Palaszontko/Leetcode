package topkfrequentelements

// Frist approach
func TopKFrequent(nums []int, k int) []int {
	hash := make(map[int]int, k)

	maxx := 0

	for _, val := range nums {
		hash[val] += 1
		maxx = max(maxx, hash[val])
	}

	array := []int{}

	for i := 0; i < k; i++ {
		for key, val := range hash {
			if val == maxx {
				array = append(array, key)
				delete(hash, key)
				maxx = 0
			}
		}

		for _, val := range hash {
			maxx = max(maxx, val)
		}
	}
	return array
}

// Second approach (way faster) -> after understanding the idea of ​​bucket sort
func TopKFrequent2(nums []int, k int) []int {
	hash := make(map[int]int, k)

	for _, val := range nums {
		hash[val] += 1
	}

	count := make([][]int, len(nums)+1)

	for key, val := range hash {
		count[val] = append(count[val], key)
	}

	final := []int{}

	for i := len(count) - 1; i >= 0; i-- {

		for j := 0; j < len(count[i]); j++ {
			final = append(final, count[i][j])

			if len(final) == k {
				return final
			}
		}
	}

	return nil
}
