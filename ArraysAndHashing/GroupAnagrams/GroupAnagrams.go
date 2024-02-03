package GroupAnagrams


import (
	"fmt"
)

func createBinaryHash(text string) int {
	hash_array := make([]int, 26)

	for _, letter := range text {
		pos := int(letter) - 'a'
		hash_array[pos] += 1
	}

	fmt.Println(text, hash_array)

	return 21
}

func GroupAnagrams(strs []string) [][]string {
    for _, val := range strs {
		createBinaryHash(val)
    }

	return nil
}


