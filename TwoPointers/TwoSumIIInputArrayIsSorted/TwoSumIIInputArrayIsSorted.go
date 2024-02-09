package twosumiiinputarrayissorted

import "fmt"

func TwoSum(numbers []int, target int) []int {

	i := 0
	j := len(numbers) - 1

	for numbers[i]+numbers[j] != target {

		fmt.Println(numbers[i], numbers[j])
		if numbers[i]+numbers[j] > target {
			j -= 1
		} else {
			i += 1
		}
	}

	return []int{i + 1, j + 1}
}
