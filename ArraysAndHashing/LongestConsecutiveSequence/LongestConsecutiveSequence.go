package LongestConsecutiveSequence

func LongestConsecutive(nums []int) int {
	hash_map := make(map[int]bool)

	for _, val := range nums {
		hash_map[val] = true
	}

	best_count := 0

	for _, val := range nums {

		if !hash_map[val-1] {
			tmp := val
			count := 1
			for hash_map[tmp+1] {
				tmp += 1
				count += 1
			}

			best_count = max(best_count, count)
		}

		if best_count > len(nums)/2 {
			break
		}
	}

	return best_count
}
