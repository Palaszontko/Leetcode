package ContainsDuplicate

func containsDuplicate(nums []int) bool {
    Hash := make(map[int]bool)

    for _, i := range nums{
        if Hash[i]{
            return true
		}else {
            Hash[i] = true;
        }
	}
	return false
}
