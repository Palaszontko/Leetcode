package ProductOfArrayExceptSelf

func ProductExceptSelf(nums []int) []int {

	prefix := make([]int, len(nums))
	sufix := make([]int, len(nums))

	copy(prefix, nums)
	copy(sufix, nums)

	for i := 1; i < len(nums); i++ {
		prefix[i] *= prefix[i-1]
		sufix[len(nums) - 1 - i] *= sufix[len(nums) - i]
	}

	array := make([]int, len(nums))
	
	array[0] = sufix[1]
	array[len(array) - 1] = prefix[len(array) - 2]


	for i := 1; i < len(nums) - 1; i++ {
		array[i] = prefix[i-1] * sufix[i + 1] 
	}

	return array

}
