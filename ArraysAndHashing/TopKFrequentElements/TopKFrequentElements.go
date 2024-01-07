package topkfrequentelements

func TopKFrequent(nums []int, k int) []int {
    hash := make(map[int]int, k)

    maxx := 0

    for _, val := range nums {
        hash[val] += 1
		maxx = max(maxx, hash[val])
    }

    array := []int{}

    for i :=0; i<k; i++{
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