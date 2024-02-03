package LongestConsecutiveSequence

func LongestConsecutive(nums []int) int {
	hash_map := make(map[int]bool)

	for _, val := range nums {
		hash_map[val] = true
	}

	list_of_starts := make([]int, 0)

	for _, val := range nums {
		if !hash_map[val-1] {
			list_of_starts = append(list_of_starts, val)
		}
	}

	best_count := 0

	for _, val := range list_of_starts {
		tmp := val
		count := 1

		for hash_map[tmp+1] {
			count += 1
			tmp += 1
		}

		best_count = max(best_count, count)
	}

	return best_count
}
