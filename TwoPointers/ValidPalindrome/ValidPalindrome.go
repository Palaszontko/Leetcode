package ValidPalindrome

import (
	"fmt"
	"strings"
)

func IsPalindrome(s string) bool {

	array := strings.ToLower(s)

	i := 0
	j := len(array) - 1

	fmt.Println(array)

	for i < j {
		fmt.Println(string(array[i]), string(array[j]))

		if (array[i] < 'a' || array[i] > 'z') && (array[i] < '0' || array[i] > '9') {
			i += 1
			continue
		}

		if (array[j] < 'a' || array[j] > 'z') && (array[j] < '0' || array[j] > '9') {
			j -= 1
			continue
		}

		if array[i] == array[j] {
			i += 1
			j -= 1
		} else {
			return false
		}

	}

	return true
}
